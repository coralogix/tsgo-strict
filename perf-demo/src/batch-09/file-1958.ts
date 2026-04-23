import type { Type1957 } from '../batch-09/file-1957.js';
export interface Type1958 {
  id: 1958;
  name: 'File1958';
  next: Type1957;
}

export function make1958(): Type1958 {
  return { id: 1958, name: 'File1958', next: null as unknown as Type1957 };
}
