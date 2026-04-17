import type { Type2010 } from '../batch-10/file-2010.js';
export interface Type2011 {
  id: 2011;
  name: 'File2011';
  next: Type2010;
}

export function make2011(): Type2011 {
  return { id: 2011, name: 'File2011', next: null as unknown as Type2010 };
}
