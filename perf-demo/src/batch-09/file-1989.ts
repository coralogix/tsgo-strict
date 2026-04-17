import type { Type1988 } from '../batch-09/file-1988.js';
export interface Type1989 {
  id: 1989;
  name: 'File1989';
  next: Type1988;
}

export function make1989(): Type1989 {
  return { id: 1989, name: 'File1989', next: null as unknown as Type1988 };
}
