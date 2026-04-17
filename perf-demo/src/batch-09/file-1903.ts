import type { Type1902 } from '../batch-09/file-1902.js';
export interface Type1903 {
  id: 1903;
  name: 'File1903';
  next: Type1902;
}

export function make1903(): Type1903 {
  return { id: 1903, name: 'File1903', next: null as unknown as Type1902 };
}
