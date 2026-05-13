import type { StorageAdapter } from "./index.js";

const DEFAULT_DB = "smritidb";
const DEFAULT_STORE = "kmf";
const ROW_KEY = "snapshot";

/**
 * IndexedDB adapter for the browser. The substrate is stored as a single
 * KMF blob in one object store. The schema is intentionally minimal — the
 * KMF wire format is the contract; the database is just a container.
 *
 * Throws if `globalThis.indexedDB` is not available (Node without a
 * `fake-indexeddb` shim; serverless edge with no IDB; older browsers).
 */
export function indexedDbAdapter(
  dbName: string = DEFAULT_DB,
  storeName: string = DEFAULT_STORE,
): StorageAdapter {
  const ensureDb = (): Promise<IDBDatabase> =>
    new Promise((resolve, reject) => {
      const idb = (globalThis as { indexedDB?: IDBFactory }).indexedDB;
      if (!idb) {
        reject(new Error("indexedDB is not available in this environment"));
        return;
      }
      const req = idb.open(dbName, 1);
      req.onupgradeneeded = () => {
        const db = req.result;
        if (!db.objectStoreNames.contains(storeName)) {
          db.createObjectStore(storeName);
        }
      };
      req.onsuccess = () => resolve(req.result);
      req.onerror = () => reject(req.error ?? new Error("indexedDB open failed"));
    });

  const txn = async <T>(
    mode: IDBTransactionMode,
    fn: (store: IDBObjectStore) => IDBRequest<T> | null,
  ): Promise<T | undefined> => {
    const db = await ensureDb();
    return new Promise<T | undefined>((resolve, reject) => {
      const tx = db.transaction(storeName, mode);
      const store = tx.objectStore(storeName);
      const req = fn(store);
      tx.oncomplete = () => resolve(req ? (req.result as T) : undefined);
      tx.onerror = () => reject(tx.error ?? new Error("indexedDB transaction failed"));
      tx.onabort = () => reject(tx.error ?? new Error("indexedDB transaction aborted"));
    });
  };

  return {
    kind: "indexeddb",

    async read(): Promise<Uint8Array | null> {
      const result = await txn("readonly", (store) => store.get(ROW_KEY));
      if (!result) return null;
      if (result instanceof Uint8Array) return result;
      if (result instanceof ArrayBuffer) return new Uint8Array(result);
      throw new Error("indexedDB returned unexpected type");
    },

    async write(bytes: Uint8Array): Promise<void> {
      await txn("readwrite", (store) => store.put(bytes, ROW_KEY));
    },

    async remove(): Promise<void> {
      await txn("readwrite", (store) => store.delete(ROW_KEY));
    },
  };
}
