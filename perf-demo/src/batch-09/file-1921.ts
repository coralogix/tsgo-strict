import type { Type1920 } from '../batch-09/file-1920.js';
export interface Type1921 {
  id: 1921;
  name: 'File1921';
  next: Type1920;
}

export function make1921(): Type1921 {
  return { id: 1921, name: 'File1921', next: null as unknown as Type1920 };
}
