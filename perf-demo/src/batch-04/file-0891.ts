import type { Type890 } from '../batch-04/file-0890.js';
export interface Type891 {
  id: 891;
  name: 'File891';
  next: Type890;
}

export function make891(): Type891 {
  return { id: 891, name: 'File891', next: null as unknown as Type890 };
}
