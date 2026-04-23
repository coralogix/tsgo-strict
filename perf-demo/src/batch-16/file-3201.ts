import type { Type3200 } from '../batch-16/file-3200.js';
export interface Type3201 {
  id: 3201;
  name: 'File3201';
  next: Type3200;
}

export function make3201(): Type3201 {
  return { id: 3201, name: 'File3201', next: null as unknown as Type3200 };
}
