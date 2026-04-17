import type { Type2130 } from '../batch-10/file-2130.js';
export interface Type2131 {
  id: 2131;
  name: 'File2131';
  next: Type2130;
}

export function make2131(): Type2131 {
  return { id: 2131, name: 'File2131', next: null as unknown as Type2130 };
}
