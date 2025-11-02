#!/usr/bin/env -S pnpm test release-images.test.js

import { getImageReleaseGroupByImageTagPrefix, } from '../release-images.mjs';
import { lazyImports, isReleaseImage, assertTagMatchesSourceVersion } from '../image-helpers.js';
describe('releaseImages', () => {
    it('gets accudo-node as the default image group', () => {
        const prefix = 'image-banana';
        const releaseGroup = getImageReleaseGroupByImageTagPrefix(prefix);
        expect(releaseGroup).toEqual('accudo-node');
    });
    it('gets indexer image group', () => {
        const prefix = 'accudo-indexer-grpc-vX.Y.Z';
        const releaseGroup = getImageReleaseGroupByImageTagPrefix(prefix);
        expect(releaseGroup).toEqual('accudo-indexer-grpc');
    });
    it('gets accudo-node as the node image group', () => {
        const prefix = 'accudo-node-vX.Y.Z';
        const releaseGroup = getImageReleaseGroupByImageTagPrefix(prefix);
        expect(releaseGroup).toEqual('accudo-node');
    });
    it('determines image is a release image', () => {
        expect(isReleaseImage("nightly-banana")).toEqual(false);
        expect(isReleaseImage("accudo-node-v1.2.3")).toEqual(true);
    });
    it('asserts version match', async () => {
        await lazyImports();
        // toThrow apparently matches a prefix, so this works but it does actually test against the real config version
        // Which... hilariously means this would fail if the version was ever 0.0.0
        expect(() => assertTagMatchesSourceVersion("accudo-node-v0.0.0")).toThrow("image tag does not match cargo version: accudo-node-v0.0.0");
    });
});
