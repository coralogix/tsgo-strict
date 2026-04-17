import type { Type940 } from '../batch-04/file-0940.js';
export interface Type941 {
  id: 941;
  name: 'File941';
  next: Type940;
}

export function make941(): Type941 {
  return { id: 941, name: 'File941', next: null as unknown as Type940 };
}
