import type { Type1978 } from '../batch-09/file-1978.js';
export interface Type1979 {
  id: 1979;
  name: 'File1979';
  next: Type1978;
}

export function make1979(): Type1979 {
  return { id: 1979, name: 'File1979', next: null as unknown as Type1978 };
}
