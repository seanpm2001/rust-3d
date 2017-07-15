/*
Copyright 2016 Martin Buck
This file is part of rust-3d.
rust-3d is free software: you can redistribute it and/or modify
it under the terms of the GNU Lesser General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.
rust-3d is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Lesser General Public License for more details.
You should have received a copy of the GNU Lesser General Public License
along with rust-3d.  If not, see <http://www.gnu.org/licenses/>.
*/

//! Result, the result type used within rust-3d. Also defining the error enum and several transformation methods between error types.

use std::result;
use std::fmt;
use std::num::ParseFloatError;
use std::num::ParseIntError;
use std::io::Error as ioError;

/// The Error Enum used by rust-3d
pub enum ErrorKind {
    MinMaxSwapped,
    MinMaxEqual,
    TooFewPoints,
    BoundingBoxMissing,
    NormalizeVecWithoutLength,
    IOError,
    ParseError,
    IndexOutOfBounds,
    IncorrectFaceID,
    IncorrectVertexID,
    IncorrectEdgeID,
    IncorrectVoxelID,
    IncorrectUnitID,
    IncorrectDimension,
    DimensionsDontMatch,
    NumberConversionError,
    NumberInWrongRange,
    ComparisionFailed,
    PlyError(PlyError)
}

pub enum PlyError {
    LoadError,
    LoadStartNotFound,
    LoadFormatNotFound,
    LoadWrongPropertyCount,
    LoadVertexIndexDefinitionNotFound,
    LoadHeaderEndNotFound,
    LoadVertexCountNotFound,
    LoadFaceCountNotFound,
    LoadVertexCountIncorrect,
    LoadVerticesIncorrect,
}

impl ErrorKind {
    /// Returns readable text for the ErrorKind
    pub fn as_str(&self) -> &'static str {
        match *self {
            ErrorKind::MinMaxSwapped             => "Passed min/max values are swapped (min > max)",
            ErrorKind::MinMaxEqual               => "Passed min/max values are equal",
            ErrorKind::TooFewPoints              => "Container had too few points for the operation",
            ErrorKind::BoundingBoxMissing        => "Bounding box is missing for the operation",
            ErrorKind::NormalizeVecWithoutLength => "Can't normalize a vector of length 0",
            ErrorKind::IOError                   => "Can't read or write a file",
            ErrorKind::ParseError                => "Can't parse data",
            ErrorKind::IndexOutOfBounds          => "Tried to access an out of bounds index",
            ErrorKind::IncorrectFaceID           => "Used an incorrect face id",
            ErrorKind::IncorrectVertexID         => "Used an incorrect vertex id",
            ErrorKind::IncorrectEdgeID           => "Used an incorrect edge id",
            ErrorKind::IncorrectVoxelID          => "Used an incorrect voxel id",
            ErrorKind::IncorrectUnitID           => "Used an incorrect unit id",
            ErrorKind::IncorrectDimension        => "Trying to access an incorrect dimension",
            ErrorKind::DimensionsDontMatch       => "Trying to mix types with different dimensions",
            ErrorKind::NumberConversionError     => "Failed converting one number type to another",
            ErrorKind::NumberInWrongRange        => "Passed number is within the wrong range",
            ErrorKind::ComparisionFailed         => "Comparision between two values failed",
            ErrorKind::PlyError(ref x)           => x.as_str()
        }
    }
}

impl PlyError {
    /// Returns readable text for the PlyError
    pub fn as_str(&self) -> &'static str {
        match *self {
            PlyError::LoadError                 => "Error while loading .ply",
            PlyError::LoadStartNotFound         => "Start of .ply header not found",
            PlyError::LoadFormatNotFound        => "Format of .ply missing or not supported",
            PlyError::LoadWrongPropertyCount    => "Property count of .ply missing or not supported",
            PlyError::LoadVertexIndexDefinitionNotFound => "Index definition in .ply not found",
            PlyError::LoadHeaderEndNotFound     => "End of header definition of .ply not found",
            PlyError::LoadVertexCountNotFound   => "Vertex count of .ply not found",
            PlyError::LoadFaceCountNotFound     => "Face count of .ply not found",
            PlyError::LoadVertexCountIncorrect  => "Vertex count of .ply not found",
            PlyError::LoadVerticesIncorrect     => "Vertices in .ply incorrect"
        }
    }
}

impl fmt::Debug for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Result type used by rust-3d
pub type Result<T> = result::Result<T, ErrorKind>;

/// Trait used to convert other Errors to ErrorKind
pub trait ToErrorKind {
    /// Creates an ErrorKind from this
    fn to_error_kind(&self) -> ErrorKind;
}

impl ToErrorKind for ParseFloatError {
    fn to_error_kind(&self) -> ErrorKind {
        ErrorKind::ParseError
    }
}

impl ToErrorKind for ParseIntError {
    fn to_error_kind(&self) -> ErrorKind {
        ErrorKind::ParseError
    }
}

impl ToErrorKind for ioError {
    fn to_error_kind(&self) -> ErrorKind {
        ErrorKind::IOError //@todo improve reporting
    }
}

impl From<ioError> for ErrorKind {
    fn from(_error: ioError) -> Self {
        ErrorKind::IOError //@todo improve reporting
    }
}
