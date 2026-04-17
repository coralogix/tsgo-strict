import type { Type2220 } from '../batch-11/file-2220.js';
export interface Type2221 {
  id: 2221;
  name: 'File2221';
  next: Type2220;
}

export function make2221(): Type2221 {
  return { id: 2221, name: 'File2221', next: null as unknown as Type2220 };
}
