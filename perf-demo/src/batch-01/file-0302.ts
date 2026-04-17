import type { Type301 } from '../batch-01/file-0301.js';
export interface Type302 {
  id: 302;
  name: 'File302';
  next: Type301;
}

export function make302(): Type302 {
  return { id: 302, name: 'File302', next: null as unknown as Type301 };
}
