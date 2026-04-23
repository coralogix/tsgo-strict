import type { Type2016 } from '../batch-10/file-2016.js';
export interface Type2017 {
  id: 2017;
  name: 'File2017';
  next: Type2016;
}

export function make2017(): Type2017 {
  return { id: 2017, name: 'File2017', next: null as unknown as Type2016 };
}
