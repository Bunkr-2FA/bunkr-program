/**
 * This code was GENERATED using the solita package.
 * Please DO NOT EDIT THIS FILE, instead rerun solita to update it or write a wrapper to add functionality.
 *
 * See: https://github.com/metaplex-foundation/solita
 */

import * as beet from '@metaplex-foundation/beet'
export type ResetRootObject = {
  newRoot: number[] /* size: 32 */
  newInitTime: number
  shadowDriveSpace: string
  resetHash: number[] /* size: 32 */
}

/**
 * @category userTypes
 * @category generated
 */
export const resetRootObjectBeet =
  new beet.FixableBeetArgsStruct<ResetRootObject>(
    [
      ['newRoot', beet.uniformFixedSizeArray(beet.u8, 32)],
      ['newInitTime', beet.u32],
      ['shadowDriveSpace', beet.utf8String],
      ['resetHash', beet.uniformFixedSizeArray(beet.u8, 32)],
    ],
    'ResetRootObject'
  )
