import type { Type1977 } from '../batch-09/file-1977.js';
export interface Type1978 {
  id: 1978;
  name: 'File1978';
  next: Type1977;
}

export function make1978(): Type1978 {
  return { id: 1978, name: 'File1978', next: null as unknown as Type1977 };
}
