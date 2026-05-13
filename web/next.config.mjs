/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
  output: "export",
  transpilePackages: ["@tanvrit/smritidb"],
  images: {
    unoptimized: true,
  },
  trailingSlash: true,
};

export default nextConfig;
