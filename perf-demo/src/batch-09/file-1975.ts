import type { Type1974 } from '../batch-09/file-1974.js';
export interface Type1975 {
  id: 1975;
  name: 'File1975';
  next: Type1974;
}

export function make1975(): Type1975 {
  return { id: 1975, name: 'File1975', next: null as unknown as Type1974 };
}
