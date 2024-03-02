import test from "ava";

import { fileURLToPath } from "url";
import { MediaPlayerThumbnail, MediaPlayerThumbnailType } from "../index.js";

test("can create file thumbnail", async (t) => {
  await t.notThrowsAsync(
    async () =>
      await MediaPlayerThumbnail.create(
        MediaPlayerThumbnailType.File,
        fileURLToPath(import.meta.url)
      )
  );
});

test("can create uri thumbnail", async (t) => {
  await t.notThrowsAsync(
    async () =>
      await MediaPlayerThumbnail.create(
        MediaPlayerThumbnailType.Uri,
        "https://via.placeholder.com/1.png"
      )
  );
});

test("cannot create unknown thumbnail", async (t) => {
  await t.throwsAsync(
    async () =>
      await MediaPlayerThumbnail.create(
        MediaPlayerThumbnailType.Unknown,
        ""
      )
  );
});
