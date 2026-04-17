import type { Type1883 } from '../batch-09/file-1883.js';
export interface Type1884 {
  id: 1884;
  name: 'File1884';
  next: Type1883;
}

export function make1884(): Type1884 {
  return { id: 1884, name: 'File1884', next: null as unknown as Type1883 };
}
