import type { Type1907 } from '../batch-09/file-1907.js';
export interface Type1908 {
  id: 1908;
  name: 'File1908';
  next: Type1907;
}

export function make1908(): Type1908 {
  return { id: 1908, name: 'File1908', next: null as unknown as Type1907 };
}
