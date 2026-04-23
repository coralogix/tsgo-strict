import type { Type2009 } from '../batch-10/file-2009.js';
export interface Type2010 {
  id: 2010;
  name: 'File2010';
  next: Type2009;
}

export function make2010(): Type2010 {
  return { id: 2010, name: 'File2010', next: null as unknown as Type2009 };
}
