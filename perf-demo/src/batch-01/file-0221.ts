import type { Type220 } from '../batch-01/file-0220.js';
export interface Type221 {
  id: 221;
  name: 'File221';
  next: Type220;
}

export function make221(): Type221 {
  return { id: 221, name: 'File221', next: null as unknown as Type220 };
}
