import type { Type3120 } from '../batch-15/file-3120.js';
export interface Type3121 {
  id: 3121;
  name: 'File3121';
  next: Type3120;
}

export function make3121(): Type3121 {
  return { id: 3121, name: 'File3121', next: null as unknown as Type3120 };
}
