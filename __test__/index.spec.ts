// TODO: Implement more tests and correctfully

import test from 'ava'

import { MediaPlayer } from '../index.js'

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