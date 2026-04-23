import type { Type1905 } from '../batch-09/file-1905.js';
export interface Type1906 {
  id: 1906;
  name: 'File1906';
  next: Type1905;
}

export function make1906(): Type1906 {
  return { id: 1906, name: 'File1906', next: null as unknown as Type1905 };
}
