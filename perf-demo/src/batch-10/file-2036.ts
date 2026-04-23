import type { Type2035 } from '../batch-10/file-2035.js';
export interface Type2036 {
  id: 2036;
  name: 'File2036';
  next: Type2035;
}

export function make2036(): Type2036 {
  return { id: 2036, name: 'File2036', next: null as unknown as Type2035 };
}
