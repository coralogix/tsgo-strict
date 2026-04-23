import type { Type160 } from '../batch-00/file-0160.js';
export interface Type161 {
  id: 161;
  name: 'File161';
  next: Type160;
}

export function make161(): Type161 {
  return { id: 161, name: 'File161', next: null as unknown as Type160 };
}
