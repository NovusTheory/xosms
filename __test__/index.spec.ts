// TODO: Implement more tests and correctfully

import test from 'ava'
import path from 'path'
import { fileURLToPath } from 'url';
const __dirname = path.dirname(fileURLToPath(import.meta.url))

import { MediaPlayer, MediaPlayerThumbnail } from '../index.js'

function getRandomServiceName() {
  return `test-xosms-${crypto.randomUUID()}`
}

test('can dispose MediaPlayer', (t) => {
  const mediaPlayer = new MediaPlayer(getRandomServiceName(), "Test Xosms");
  t.notThrows(() => mediaPlayer.dispose());
})

test('can activate and deactivate MediaPlayer', async (t) => {
  const mediaPlayer = new MediaPlayer(getRandomServiceName(), "Test Xosms");
  await t.notThrowsAsync(async () => await mediaPlayer.activate());
  await t.notThrowsAsync(async () => await mediaPlayer.deactivate());
  mediaPlayer.dispose();
})

test('cannot use MediaPlayer backend after dispose', async (t) => {
  const mediaPlayer = new MediaPlayer(getRandomServiceName(), "Test Xosms");
  mediaPlayer.dispose();
  await t.throwsAsync(async () => await mediaPlayer.activate());
  await t.throwsAsync(async () => await mediaPlayer.update());
  await t.throwsAsync(async () => await mediaPlayer.deactivate());
  await t.throwsAsync(async () => await mediaPlayer.setTimeline(0, 0));
})

test('can set and update all MediaPlayer properties', async (t) => {
  const mediaPlayer = new MediaPlayer(getRandomServiceName(), "Test Xosms");
  mediaPlayer.albumTitle = "test";
  mediaPlayer.artist = ["test", "test"];
  mediaPlayer.fullscreen = true;
  mediaPlayer.loop = "track";
  mediaPlayer.maximumPlaybackRate = 2.0;
  mediaPlayer.minimumPlaybackRate = 0.25;
  mediaPlayer.nextButtonEnabled = true;
  mediaPlayer.pauseButtonEnabled = true;
  mediaPlayer.playButtonEnabled = true;
  mediaPlayer.playbackRate = 0.75;
  mediaPlayer.playbackStatus = "playing";
  mediaPlayer.previousButtonEnabled = true;
  mediaPlayer.seekEnabled = true;
  mediaPlayer.shuffle = true;
  mediaPlayer.stopButtonEnabled = true;
  mediaPlayer.thumbnail = await MediaPlayerThumbnail.create("uri", "https://placehold.co/1x1.png");
  mediaPlayer.title = "test";
  mediaPlayer.trackId = "test";
  mediaPlayer.volume = 0.5;
  await mediaPlayer.activate();
  await t.notThrowsAsync(async () => await mediaPlayer.update());
  await t.notThrowsAsync(async () => await mediaPlayer.setTimeline(60, 30));
  mediaPlayer.dispose();
})

test('can create MediaPlayerThumbnail of all types', async (t) => {
  await t.notThrowsAsync(async () => await MediaPlayerThumbnail.create("uri", "https://placehold.co/1x1.png"));
  await t.notThrowsAsync(async () => await MediaPlayerThumbnail.create("file", path.resolve(__dirname, "./thumbnail_test.png")));
})