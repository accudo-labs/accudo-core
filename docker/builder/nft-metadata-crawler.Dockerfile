### NFT Metadata Crawler Image ###

FROM indexer-builder

FROM debian-base AS nft-metadata-crawler

COPY --link --from=indexer-builder /accudo/dist/accudo-nft-metadata-crawler /usr/local/bin/accudo-nft-metadata-crawler

# The health check port
EXPOSE 8080
