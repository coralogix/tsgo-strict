import type { Type2007 } from '../batch-10/file-2007.js';
export interface Type2008 {
  id: 2008;
  name: 'File2008';
  next: Type2007;
}

export function make2008(): Type2008 {
  return { id: 2008, name: 'File2008', next: null as unknown as Type2007 };
}
