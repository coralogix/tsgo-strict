import type { Type3500 } from '../batch-17/file-3500.js';
export interface Type3501 {
  id: 3501;
  name: 'File3501';
  next: Type3500;
}

export function make3501(): Type3501 {
  return { id: 3501, name: 'File3501', next: null as unknown as Type3500 };
}
