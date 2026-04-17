import type { Type1890 } from '../batch-09/file-1890.js';
export interface Type1891 {
  id: 1891;
  name: 'File1891';
  next: Type1890;
}

export function make1891(): Type1891 {
  return { id: 1891, name: 'File1891', next: null as unknown as Type1890 };
}
