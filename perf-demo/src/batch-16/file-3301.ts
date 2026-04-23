import type { Type3300 } from '../batch-16/file-3300.js';
export interface Type3301 {
  id: 3301;
  name: 'File3301';
  next: Type3300;
}

export function make3301(): Type3301 {
  return { id: 3301, name: 'File3301', next: null as unknown as Type3300 };
}
