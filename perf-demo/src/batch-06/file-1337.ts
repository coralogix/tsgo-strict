import type { Type1336 } from '../batch-06/file-1336.js';
export interface Type1337 {
  id: 1337;
  name: 'File1337';
  next: Type1336;
}

export function make1337(): Type1337 {
  return { id: 1337, name: 'File1337', next: null as unknown as Type1336 };
}
