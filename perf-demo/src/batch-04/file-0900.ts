import type { Type899 } from '../batch-04/file-0899.js';
export interface Type900 {
  id: 900;
  name: 'File900';
  next: Type899;
}

export function make900(): Type900 {
  return { id: 900, name: 'File900', next: null as unknown as Type899 };
}
