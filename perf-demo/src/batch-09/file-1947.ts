import type { Type1946 } from '../batch-09/file-1946.js';
export interface Type1947 {
  id: 1947;
  name: 'File1947';
  next: Type1946;
}

export function make1947(): Type1947 {
  return { id: 1947, name: 'File1947', next: null as unknown as Type1946 };
}
