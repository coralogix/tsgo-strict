import type { Type1211 } from '../batch-06/file-1211.js';
export interface Type1212 {
  id: 1212;
  name: 'File1212';
  next: Type1211;
}

export function make1212(): Type1212 {
  return { id: 1212, name: 'File1212', next: null as unknown as Type1211 };
}
