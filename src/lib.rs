use std::{error::Error, fmt::Display};

use clap::ValueEnum;

const ANY: u8 = b'*';

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, strum::EnumIter, ValueEnum,
)]
pub enum PatchTyp {
    /// Enables Ironman with any checksum
    ModdedIronman,
    /// Enables loading saves in ironman and makes ironman saves available in
    /// that manu
    EnableIronmanLoading,
    /// Turns normal saves into ironman saves by hovering over the "load"
    /// button during gameplay. This is very unuseable, as saves converted
    /// with this method do not have any possisble achievements listed.
    /// This is mainly just for testing.
    MidgameIronman,
}

#[derive(Debug, Clone)]
struct PatchPart {
    target: &'static [u8],
    location: usize,
    pre_patch: &'static [u8],
    post_patch: &'static [u8],
}

#[derive(Debug, Clone)]
pub struct Patch {
    typ: PatchTyp,
    parts: Vec<PatchPart>,
}

impl Patch {
    pub fn typ(&self) -> PatchTyp {
        self.typ
    }

    pub fn apply(&self, game: &mut [u8]) -> Result<(), PatchError> {
        for part in &self.parts {
            if game.len() < part.location + part.pre_patch.len() {
                return Err(PatchError::PatchFileDoesNotMatchTarget);
            }
            if !instruction_eq(
                &game[part.location..part.location + part.pre_patch.len()],
                part.pre_patch,
            ) {
                return Err(PatchError::PatchFileDoesNotMatchTarget);
            }
        }

        for part in &self.parts {
            game[part.location..part.location + part.post_patch.len()]
                .copy_from_slice(part.post_patch)
        }

        Ok(())
    }
}

#[derive(Debug)]
pub enum PatchError {
    Unavailable,
    AlreadyApplied,
    PatchFileDoesNotMatchTarget,
    MultiplePossibleLocations,
}

impl Display for PatchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PatchError::Unavailable => {
                f.write_str("Could not find a patch for this patch type")
            }
            PatchError::AlreadyApplied => f.write_str(
                "This patch type is already applied to the executable",
            ),
            PatchError::PatchFileDoesNotMatchTarget => f.write_str(
                "The file used to calculate the patch and apply the patch are \
                 not the same",
            ),
            PatchError::MultiplePossibleLocations => f.write_str(
                "Could not find a patch, as there are multiple possible \
                 locations",
            ),
        }
    }
}

impl Error for PatchError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}

impl PatchTyp {
    pub fn find_patch(&self, game: &[u8]) -> Result<Patch, PatchError> {
        let mut patches = match self {
            PatchTyp::ModdedIronman => {
                let target = &[
                    0x01, 0x48, 0x8D, 0x97, 0x58, 0x02, 0x00, 0x00, 0x48, 0x83,
                    0x7A, 0x18, 0x10, 0x72, 0x03,
                ];

                // settz bl
                let pre_patch = &[0x0F, 0x94, 0xC3];
                // inc
                let post_patch = &[0x40, 0xFE, 0xC3];

                vec![PatchPart {
                    target,
                    location: 0,
                    pre_patch,
                    post_patch,
                }]
            }
            PatchTyp::EnableIronmanLoading => {
                let target = &[
                    0xD2, 0x48, 0x8B, 0x01, 0x4C, 0x8B, 0x80, 0x80, 0x00, 0x00,
                    0x00, 0x4C, 0x3B, 0xC7, 0x75, 0x14, 0x84, 0xD2, 0x74, 0x08,
                ];
                let pre_patch = &[0x74, 0x08];
                let post_patch = &[0x90; 2];

                let a = PatchPart {
                    target,
                    location: 0,
                    pre_patch,
                    post_patch,
                };

                let target = &[
                    0xD7, 0x49, 0x8B, 0xCC, 0x41, 0xFF, 0x50, 0x28, 0x84, 0xC0,
                    0x0F, 0x84, 0x9C, 0x00, 0x00, 0x00,
                ];
                let pre_patch = &[0x0F, 0x84, 0x9C, 0x00, 0x00, 0x00];
                let post_patch = &[0x90; 6];
                let b = PatchPart {
                    target,
                    location: 0,
                    pre_patch,
                    post_patch,
                };

                vec![a, b]
            }
            PatchTyp::MidgameIronman => {
                let target = &[
                    0x48, 0x8b, 0x05, ANY, ANY, ANY, ANY, 0x80, 0xb8, ANY,
                    0x24, 0x00, 0x00, 0x00, 0x74, 0x0C,
                ];
                let pre_patch =
                    &[0x80, 0xb8, ANY, 0x24, 0x00, 0x00, 0x00, 0x74];
                let post_patch =
                    &[0xC6, 0x80, 0xF0, 0x24, 0x00, 0x00, 0x00, 0xEB];

                vec![PatchPart {
                    target,
                    location: 0,
                    pre_patch,
                    post_patch,
                }]
            }
        };

        for PatchPart {
            target,
            location,
            pre_patch,
            post_patch,
        } in &mut patches
        {
            *location = find_location(game, target, pre_patch, post_patch)?;
            assert!(pre_patch.len() == post_patch.len());
        }

        Ok(Patch {
            parts: patches,
            typ: *self,
        })
    }
}

fn instruction_eq(location: &[u8], target: &[u8]) -> bool {
    if location.len() != target.len() {
        return false;
    }

    for (l, t) in location.iter().zip(target) {
        if *t != ANY && t != l {
            return false;
        }
    }

    true
}

fn find_location(
    game: &[u8],
    target: &[u8],
    pre_patch: &[u8],
    post_patch: &[u8],
) -> Result<usize, PatchError> {
    let mut tf = None;
    for (offset, chunk) in game.windows(target.len()).enumerate() {
        if instruction_eq(chunk, target) {
            if tf.is_some() {
                return Err(PatchError::MultiplePossibleLocations);
            }
            tf = Some(offset)
        }
    }
    let tf = tf.ok_or(PatchError::Unavailable)?;
    let mut to = None;
    for (offset, chunk) in
        game[tf..tf + 100].windows(pre_patch.len()).enumerate()
    {
        if instruction_eq(chunk, pre_patch) {
            to = Some(offset);
            break;
        }
        if chunk == post_patch {
            // already patches
            return Err(PatchError::AlreadyApplied);
        }
    }
    let to = to.ok_or(PatchError::Unavailable)?;
    let to = to + tf;
    Ok(to)
}
