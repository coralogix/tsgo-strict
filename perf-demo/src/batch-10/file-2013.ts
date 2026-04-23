import type { Type2012 } from '../batch-10/file-2012.js';
export interface Type2013 {
  id: 2013;
  name: 'File2013';
  next: Type2012;
}

export function make2013(): Type2013 {
  return { id: 2013, name: 'File2013', next: null as unknown as Type2012 };
}
