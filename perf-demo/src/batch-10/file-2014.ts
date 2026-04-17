import type { Type2013 } from '../batch-10/file-2013.js';
export interface Type2014 {
  id: 2014;
  name: 'File2014';
  next: Type2013;
}

export function make2014(): Type2014 {
  return { id: 2014, name: 'File2014', next: null as unknown as Type2013 };
}
