import type { Type2221 } from '../batch-11/file-2221.js';
export interface Type2222 {
  id: 2222;
  name: 'File2222';
  next: Type2221;
}

export function make2222(): Type2222 {
  return { id: 2222, name: 'File2222', next: null as unknown as Type2221 };
}
