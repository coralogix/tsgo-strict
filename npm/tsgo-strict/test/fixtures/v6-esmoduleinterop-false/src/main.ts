import fs from 'node:fs';

export function readFileSync(path) {
  return fs.readFileSync(path, 'utf8');
}
