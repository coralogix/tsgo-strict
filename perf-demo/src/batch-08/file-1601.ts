import type { Type1600 } from '../batch-08/file-1600.js';
export interface Type1601 {
  id: 1601;
  name: 'File1601';
  next: Type1600;
}

export function make1601(): Type1601 {
  return { id: 1601, name: 'File1601', next: null as unknown as Type1600 };
}
