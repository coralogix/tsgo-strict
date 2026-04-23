import type { Type127 } from '../batch-00/file-0127.js';
export interface Type128 {
  id: 128;
  name: 'File128';
  next: Type127;
}

export function make128(): Type128 {
  return { id: 128, name: 'File128', next: null as unknown as Type127 };
}
