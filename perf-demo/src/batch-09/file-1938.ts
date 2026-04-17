import type { Type1937 } from '../batch-09/file-1937.js';
export interface Type1938 {
  id: 1938;
  name: 'File1938';
  next: Type1937;
}

export function make1938(): Type1938 {
  return { id: 1938, name: 'File1938', next: null as unknown as Type1937 };
}
