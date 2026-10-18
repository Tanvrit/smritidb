#!/usr/bin/env python3
"""Copy generated UniFFI bindings while preserving Kotlin compatibility fixes.

Run after `uniffi-bindgen generate ... --language kotlin`. Layout changes fail
closed so regenerating cannot silently drop the AutoCloseable or exception fix.
"""
from pathlib import Path
import re
r=Path(__file__).resolve().parent.parent;source=r/'packages/smritidb-ffi/bindings/kotlin/uniffi/smritidb/smritidb.kt';dest=r/'packages/smritidb-kmp/src/jvmMain/kotlin/uniffi/smritidb/smritidb.kt';s=source.read_text();assert s.count('  fun `close`()\n')==1, 'UniFFI close interface changed';s=s.replace('  fun `close`()\n','',1);s=s.replace('  override fun `close`() =','  private fun closeFfi() =',1)
old='''  @Synchronized
  override fun close() {
    this.destroy()
  }''';new='''  @Synchronized
  override fun close() {
    // Release the adapter before destroying UniFFI's handle. Close is idempotent.
    if (!wasDestroyed.get()) {
      try { closeFfi() } catch (_: Throwable) { /* Preserve AutoCloseable compatibility. */ }
    }
    this.destroy()
  }''';assert s.count(old)==2;s=s.replace(old,new,1);s=s.replace('val `message`: kotlin.String','val `msg`: kotlin.String').replace('${ `message` }','${ `msg` }').replace('value.`message`','value.`msg`');s=s.replace('@file:Suppress','// Copied from the UniFFI output with local Kotlin compatibility patches:\n// fold the UDL close into AutoCloseable.close; keep exception payload as msg.\n\n@file:Suppress',1);dest.write_text(s)
