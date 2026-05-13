import type { Metadata } from "next";
import "./globals.css";
import { Nav } from "../components/Nav";
import { Footer } from "../components/Footer";

export const metadata: Metadata = {
  title: "Kanerva — an open associative memory standard",
  description:
    "Kanerva is a biology-inspired associative memory layer for every platform. Storage that remembers by partial cue, degrades like a hologram, and consolidates the way you do.",
  metadataBase: new URL("https://kanervalabs.com"),
};

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="en">
      <head>
        <link rel="preconnect" href="https://fonts.googleapis.com" />
        <link rel="preconnect" href="https://fonts.gstatic.com" crossOrigin="" />
        <link
          rel="stylesheet"
          href="https://fonts.googleapis.com/css2?family=Spectral:ital,wght@0,300;0,400;0,500;0,600;0,700;1,400&family=IBM+Plex+Mono:wght@300;400;500&display=swap"
        />
      </head>
      <body className="min-h-screen grain">
        <Nav />
        <main>{children}</main>
        <Footer />
      </body>
    </html>
  );
}
