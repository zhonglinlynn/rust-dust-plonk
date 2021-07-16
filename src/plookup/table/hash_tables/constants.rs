// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.

// For the Bls and BN254 curves, the large prime p is different.
// This leads to diffences in the subsequent difference in constants
// we have defined below.
// These are the required constants for the
// Currently making the s_i usize.

use bigint::U256 as u256;
use dusk_bls12_381::BlsScalar;

/// Constant V
pub const V: usize = 643;
/// Constant N
pub const N: u64 = 27;
/// Note this is currently backwards, e.g. S[0] should = 673. But doesn't matter for now
pub const S: [u64; 27] = [
    651, 658, 656, 666, 663, 654, 668, 677, 681, 683, 669, 681, 680, 677, 675, 668, 675, 683, 681,
    683, 683, 655, 680, 683, 667, 678, 673,
];
/// Constant T_s
pub const T_S: u32 = 4;

/// Montgomery form of the s_i, needed to input them as variables
pub const S_I_DECOMPOSITION_MONTGOMERY: [BlsScalar; 27] = [
    BlsScalar([
        6571299961350,
        9554885865219436026,
        15585515709981716521,
        2712848100111761021,
    ]),
    BlsScalar([
        6597069765120,
        10243417561410045440,
        11884946391852638217,
        7931979164453719756,
    ]),
    BlsScalar([
        6579889895940,
        15933311121852823036,
        8203077912702173214,
        4452558454892413933,
    ]),
    BlsScalar([
        6335076760125,
        9392260008042033603,
        15688370324364089678,
        987673527917684992,
    ]),
    BlsScalar([
        6438155975205,
        12146386792804471259,
        886093051847776462,
        3417453711575968316,
    ]),
    BlsScalar([
        6588479830530,
        3864992304776658430,
        820640115422629908,
        6192268809673066845,
    ]),
    BlsScalar([
        6549825124875,
        2832194760490744309,
        15594866129471023172,
        7586944250014904550,
    ]),
    BlsScalar([
        6571299961350,
        9554885865219436026,
        15585515709981716521,
        2712848100111761021,
    ]),
    BlsScalar([
        6635724470775,
        11276215105695959561,
        15557464451513796569,
        6537303724111882050,
    ]),
    BlsScalar([
        6524055321105,
        2143663064300134895,
        848691373890549860,
        2367813185672945816,
    ]),
    BlsScalar([
        6635724470775,
        11276215105695959561,
        15557464451513796569,
        6537303724111882050,
    ]),
    BlsScalar([
        6579889895940,
        15933311121852823036,
        8203077912702173214,
        4452558454892413933,
    ]),
    BlsScalar([
        6644314405365,
        17654640362329346571,
        8175026654234253262,
        8277014078892534962,
    ]),
    BlsScalar([
        6579889895940,
        15933311121852823036,
        8203077912702173214,
        4452558454892413933,
    ]),
    BlsScalar([
        6627134536185,
        4897789849062572551,
        4493158175083788260,
        4797593369331229139,
    ]),
    BlsScalar([
        6644314405365,
        17654640362329346571,
        8175026654234253262,
        8277014078892534962,
    ]),
    BlsScalar([
        6644314405365,
        17654640362329346571,
        8175026654234253262,
        8277014078892534962,
    ]),
    BlsScalar([
        6644314405365,
        17654640362329346571,
        8175026654234253262,
        8277014078892534962,
    ]),
    BlsScalar([
        6588479830530,
        3864992304776658430,
        820640115422629908,
        6192268809673066845,
    ]),
    BlsScalar([
        6618544601595,
        16966108666138737157,
        11875595972363331566,
        3057883014550576227,
    ]),
    BlsScalar([
        6609954667005,
        10587683409505350147,
        811289695933323257,
        1318172659769923316,
    ]),
    BlsScalar([
        6665789241840,
        5930587393348486672,
        8165676234744946612,
        3402917928989391433,
    ]),
    BlsScalar([
        6657199307250,
        17998906210424651278,
        15548114032024489918,
        1663207574208738521,
    ]),
    BlsScalar([
        6549825124875,
        2832194760490744309,
        15594866129471023172,
        7586944250014904550,
    ]),
    BlsScalar([
        6524055321105,
        2143663064300134895,
        848691373890549860,
        2367813185672945816,
    ]),
    BlsScalar([
        6665789241840,
        5930587393348486672,
        8165676234744946612,
        3402917928989391433,
    ]),
    BlsScalar([
        6438155975205,
        12146386792804471259,
        886093051847776462,
        3417453711575968316,
    ]),
];

/// Divisors used in modular reduciton via precomputation
pub const BLS_DIVISORS: [u64; 26] = [
    12483978167071014912,
    12538021362599460864,
    12501992565580496896,
    12033618204333965312,
    12231776587938267136,
    12520006964089978880,
    12447949370052050944,
    12483978167071014912,
    12610078956637388800,
    12393906174523604992,
    12610078956637388800,
    12501992565580496896,
    12628093355146870784,
    12501992565580496896,
    12592064558127906816,
    12628093355146870784,
    12628093355146870784,
    12628093355146870784,
    12520006964089978880,
    12574050159618424832,
    12556035761108942848,
    12664122152165834752,
    12646107753656352768,
    12447949370052050944,
    12393906174523604992,
    12664122152165834752,
];

/// Reciprocals used in modular reduciton via precomputation
pub const BLS_RECIP: [u64; 26] = [
    8810782522940637207,
    8693293184161972600,
    8771506548017510134,
    9830899536288323915,
    9372793380603527698,
    8732343597482651052,
    8889675508748597233,
    8810782522940637207,
    8538207256974135319,
    9008875012741874045,
    8538207256974135319,
    8771506548017510134,
    8499712319269878989,
    8771506548017510134,
    8576812337561665629,
    8499712319269878989,
    8499712319269878989,
    8499712319269878989,
    8732343597482651052,
    8615528034425951041,
    8654354823677221489,
    8423050992405072644,
    8461327053752814274,
    8889675508748597233,
    9008875012741874045,
    8423050992405072644,
];

/// Predefined S-box
pub const SBOX_U256: [u256; 659] = [
    u256([15, 0, 0, 0]),
    u256([187, 0, 0, 0]),
    u256([186, 0, 0, 0]),
    u256([168, 0, 0, 0]),
    u256([493, 0, 0, 0]),
    u256([102, 0, 0, 0]),
    u256([296, 0, 0, 0]),
    u256([11, 0, 0, 0]),
    u256([164, 0, 0, 0]),
    u256([155, 0, 0, 0]),
    u256([527, 0, 0, 0]),
    u256([103, 0, 0, 0]),
    u256([192, 0, 0, 0]),
    u256([589, 0, 0, 0]),
    u256([543, 0, 0, 0]),
    u256([450, 0, 0, 0]),
    u256([647, 0, 0, 0]),
    u256([72, 0, 0, 0]),
    u256([343, 0, 0, 0]),
    u256([386, 0, 0, 0]),
    u256([279, 0, 0, 0]),
    u256([616, 0, 0, 0]),
    u256([225, 0, 0, 0]),
    u256([140, 0, 0, 0]),
    u256([313, 0, 0, 0]),
    u256([586, 0, 0, 0]),
    u256([276, 0, 0, 0]),
    u256([57, 0, 0, 0]),
    u256([162, 0, 0, 0]),
    u256([68, 0, 0, 0]),
    u256([179, 0, 0, 0]),
    u256([445, 0, 0, 0]),
    u256([418, 0, 0, 0]),
    u256([364, 0, 0, 0]),
    u256([46, 0, 0, 0]),
    u256([591, 0, 0, 0]),
    u256([541, 0, 0, 0]),
    u256([218, 0, 0, 0]),
    u256([0, 0, 0, 0]),
    u256([437, 0, 0, 0]),
    u256([618, 0, 0, 0]),
    u256([157, 0, 0, 0]),
    u256([657, 0, 0, 0]),
    u256([49, 0, 0, 0]),
    u256([120, 0, 0, 0]),
    u256([469, 0, 0, 0]),
    u256([142, 0, 0, 0]),
    u256([325, 0, 0, 0]),
    u256([183, 0, 0, 0]),
    u256([123, 0, 0, 0]),
    u256([23, 0, 0, 0]),
    u256([468, 0, 0, 0]),
    u256([619, 0, 0, 0]),
    u256([217, 0, 0, 0]),
    u256([472, 0, 0, 0]),
    u256([226, 0, 0, 0]),
    u256([212, 0, 0, 0]),
    u256([406, 0, 0, 0]),
    u256([4, 0, 0, 0]),
    u256([499, 0, 0, 0]),
    u256([182, 0, 0, 0]),
    u256([51, 0, 0, 0]),
    u256([141, 0, 0, 0]),
    u256([86, 0, 0, 0]),
    u256([596, 0, 0, 0]),
    u256([70, 0, 0, 0]),
    u256([149, 0, 0, 0]),
    u256([355, 0, 0, 0]),
    u256([351, 0, 0, 0]),
    u256([245, 0, 0, 0]),
    u256([52, 0, 0, 0]),
    u256([193, 0, 0, 0]),
    u256([311, 0, 0, 0]),
    u256([244, 0, 0, 0]),
    u256([375, 0, 0, 0]),
    u256([300, 0, 0, 0]),
    u256([399, 0, 0, 0]),
    u256([590, 0, 0, 0]),
    u256([143, 0, 0, 0]),
    u256([24, 0, 0, 0]),
    u256([190, 0, 0, 0]),
    u256([517, 0, 0, 0]),
    u256([208, 0, 0, 0]),
    u256([539, 0, 0, 0]),
    u256([544, 0, 0, 0]),
    u256([236, 0, 0, 0]),
    u256([393, 0, 0, 0]),
    u256([34, 0, 0, 0]),
    u256([203, 0, 0, 0]),
    u256([60, 0, 0, 0]),
    u256([151, 0, 0, 0]),
    u256([243, 0, 0, 0]),
    u256([542, 0, 0, 0]),
    u256([299, 0, 0, 0]),
    u256([368, 0, 0, 0]),
    u256([289, 0, 0, 0]),
    u256([272, 0, 0, 0]),
    u256([567, 0, 0, 0]),
    u256([280, 0, 0, 0]),
    u256([599, 0, 0, 0]),
    u256([625, 0, 0, 0]),
    u256([341, 0, 0, 0]),
    u256([462, 0, 0, 0]),
    u256([509, 0, 0, 0]),
    u256([153, 0, 0, 0]),
    u256([374, 0, 0, 0]),
    u256([213, 0, 0, 0]),
    u256([477, 0, 0, 0]),
    u256([310, 0, 0, 0]),
    u256([347, 0, 0, 0]),
    u256([572, 0, 0, 0]),
    u256([71, 0, 0, 0]),
    u256([579, 0, 0, 0]),
    u256([158, 0, 0, 0]),
    u256([412, 0, 0, 0]),
    u256([587, 0, 0, 0]),
    u256([63, 0, 0, 0]),
    u256([172, 0, 0, 0]),
    u256([640, 0, 0, 0]),
    u256([173, 0, 0, 0]),
    u256([101, 0, 0, 0]),
    u256([439, 0, 0, 0]),
    u256([5, 0, 0, 0]),
    u256([92, 0, 0, 0]),
    u256([501, 0, 0, 0]),
    u256([500, 0, 0, 0]),
    u256([330, 0, 0, 0]),
    u256([633, 0, 0, 0]),
    u256([630, 0, 0, 0]),
    u256([328, 0, 0, 0]),
    u256([488, 0, 0, 0]),
    u256([356, 0, 0, 0]),
    u256([89, 0, 0, 0]),
    u256([224, 0, 0, 0]),
    u256([383, 0, 0, 0]),
    u256([96, 0, 0, 0]),
    u256([643, 0, 0, 0]),
    u256([585, 0, 0, 0]),
    u256([422, 0, 0, 0]),
    u256([41, 0, 0, 0]),
    u256([295, 0, 0, 0]),
    u256([642, 0, 0, 0]),
    u256([571, 0, 0, 0]),
    u256([247, 0, 0, 0]),
    u256([239, 0, 0, 0]),
    u256([600, 0, 0, 0]),
    u256([561, 0, 0, 0]),
    u256([319, 0, 0, 0]),
    u256([480, 0, 0, 0]),
    u256([570, 0, 0, 0]),
    u256([652, 0, 0, 0]),
    u256([134, 0, 0, 0]),
    u256([620, 0, 0, 0]),
    u256([484, 0, 0, 0]),
    u256([525, 0, 0, 0]),
    u256([333, 0, 0, 0]),
    u256([177, 0, 0, 0]),
    u256([209, 0, 0, 0]),
    u256([8, 0, 0, 0]),
    u256([211, 0, 0, 0]),
    u256([402, 0, 0, 0]),
    u256([478, 0, 0, 0]),
    u256([574, 0, 0, 0]),
    u256([148, 0, 0, 0]),
    u256([365, 0, 0, 0]),
    u256([83, 0, 0, 0]),
    u256([635, 0, 0, 0]),
    u256([44, 0, 0, 0]),
    u256([646, 0, 0, 0]),
    u256([204, 0, 0, 0]),
    u256([414, 0, 0, 0]),
    u256([413, 0, 0, 0]),
    u256([398, 0, 0, 0]),
    u256([449, 0, 0, 0]),
    u256([363, 0, 0, 0]),
    u256([588, 0, 0, 0]),
    u256([65, 0, 0, 0]),
    u256([617, 0, 0, 0]),
    u256([658, 0, 0, 0]),
    u256([126, 0, 0, 0]),
    u256([178, 0, 0, 0]),
    u256([536, 0, 0, 0]),
    u256([624, 0, 0, 0]),
    u256([201, 0, 0, 0]),
    u256([513, 0, 0, 0]),
    u256([506, 0, 0, 0]),
    u256([384, 0, 0, 0]),
    u256([336, 0, 0, 0]),
    u256([382, 0, 0, 0]),
    u256([348, 0, 0, 0]),
    u256([223, 0, 0, 0]),
    u256([316, 0, 0, 0]),
    u256([629, 0, 0, 0]),
    u256([88, 0, 0, 0]),
    u256([18, 0, 0, 0]),
    u256([278, 0, 0, 0]),
    u256([287, 0, 0, 0]),
    u256([524, 0, 0, 0]),
    u256([257, 0, 0, 0]),
    u256([421, 0, 0, 0]),
    u256([639, 0, 0, 0]),
    u256([424, 0, 0, 0]),
    u256([452, 0, 0, 0]),
    u256([511, 0, 0, 0]),
    u256([564, 0, 0, 0]),
    u256([538, 0, 0, 0]),
    u256([214, 0, 0, 0]),
    u256([514, 0, 0, 0]),
    u256([307, 0, 0, 0]),
    u256([31, 0, 0, 0]),
    u256([93, 0, 0, 0]),
    u256([471, 0, 0, 0]),
    u256([104, 0, 0, 0]),
    u256([528, 0, 0, 0]),
    u256([234, 0, 0, 0]),
    u256([352, 0, 0, 0]),
    u256([255, 0, 0, 0]),
    u256([534, 0, 0, 0]),
    u256([580, 0, 0, 0]),
    u256([113, 0, 0, 0]),
    u256([360, 0, 0, 0]),
    u256([526, 0, 0, 0]),
    u256([614, 0, 0, 0]),
    u256([532, 0, 0, 0]),
    u256([603, 0, 0, 0]),
    u256([537, 0, 0, 0]),
    u256([1, 0, 0, 0]),
    u256([370, 0, 0, 0]),
    u256([121, 0, 0, 0]),
    u256([430, 0, 0, 0]),
    u256([32, 0, 0, 0]),
    u256([417, 0, 0, 0]),
    u256([426, 0, 0, 0]),
    u256([391, 0, 0, 0]),
    u256([644, 0, 0, 0]),
    u256([358, 0, 0, 0]),
    u256([206, 0, 0, 0]),
    u256([3, 0, 0, 0]),
    u256([504, 0, 0, 0]),
    u256([13, 0, 0, 0]),
    u256([557, 0, 0, 0]),
    u256([444, 0, 0, 0]),
    u256([284, 0, 0, 0]),
    u256([584, 0, 0, 0]),
    u256([39, 0, 0, 0]),
    u256([251, 0, 0, 0]),
    u256([176, 0, 0, 0]),
    u256([508, 0, 0, 0]),
    u256([94, 0, 0, 0]),
    u256([156, 0, 0, 0]),
    u256([33, 0, 0, 0]),
    u256([273, 0, 0, 0]),
    u256([496, 0, 0, 0]),
    u256([246, 0, 0, 0]),
    u256([321, 0, 0, 0]),
    u256([58, 0, 0, 0]),
    u256([21, 0, 0, 0]),
    u256([165, 0, 0, 0]),
    u256([638, 0, 0, 0]),
    u256([436, 0, 0, 0]),
    u256([10, 0, 0, 0]),
    u256([145, 0, 0, 0]),
    u256([194, 0, 0, 0]),
    u256([498, 0, 0, 0]),
    u256([267, 0, 0, 0]),
    u256([292, 0, 0, 0]),
    u256([90, 0, 0, 0]),
    u256([497, 0, 0, 0]),
    u256([505, 0, 0, 0]),
    u256([510, 0, 0, 0]),
    u256([80, 0, 0, 0]),
    u256([435, 0, 0, 0]),
    u256([303, 0, 0, 0]),
    u256([42, 0, 0, 0]),
    u256([533, 0, 0, 0]),
    u256([529, 0, 0, 0]),
    u256([453, 0, 0, 0]),
    u256([329, 0, 0, 0]),
    u256([428, 0, 0, 0]),
    u256([35, 0, 0, 0]),
    u256([337, 0, 0, 0]),
    u256([269, 0, 0, 0]),
    u256([229, 0, 0, 0]),
    u256([297, 0, 0, 0]),
    u256([85, 0, 0, 0]),
    u256([562, 0, 0, 0]),
    u256([440, 0, 0, 0]),
    u256([357, 0, 0, 0]),
    u256([95, 0, 0, 0]),
    u256([50, 0, 0, 0]),
    u256([559, 0, 0, 0]),
    u256([446, 0, 0, 0]),
    u256([656, 0, 0, 0]),
    u256([606, 0, 0, 0]),
    u256([457, 0, 0, 0]),
    u256([459, 0, 0, 0]),
    u256([390, 0, 0, 0]),
    u256([59, 0, 0, 0]),
    u256([611, 0, 0, 0]),
    u256([306, 0, 0, 0]),
    u256([623, 0, 0, 0]),
    u256([188, 0, 0, 0]),
    u256([650, 0, 0, 0]),
    u256([582, 0, 0, 0]),
    u256([170, 0, 0, 0]),
    u256([249, 0, 0, 0]),
    u256([16, 0, 0, 0]),
    u256([380, 0, 0, 0]),
    u256([230, 0, 0, 0]),
    u256([130, 0, 0, 0]),
    u256([169, 0, 0, 0]),
    u256([138, 0, 0, 0]),
    u256([612, 0, 0, 0]),
    u256([207, 0, 0, 0]),
    u256([227, 0, 0, 0]),
    u256([598, 0, 0, 0]),
    u256([47, 0, 0, 0]),
    u256([483, 0, 0, 0]),
    u256([73, 0, 0, 0]),
    u256([67, 10, 0, 0]),
    u256([106, 0, 0, 0]),
    u256([175, 0, 0, 0]),
    u256([655, 0, 0, 0]),
    u256([22, 0, 0, 0]),
    u256([77, 0, 0, 0]),
    u256([133, 0, 0, 0]),
    u256([283, 0, 0, 0]),
    u256([377, 0, 0, 0]),
    u256([112, 0, 0, 0]),
    u256([232, 0, 0, 0]),
    u256([429, 0, 0, 0]),
    u256([117, 0, 0, 0]),
    u256([111, 0, 0, 0]),
    u256([332, 0, 0, 0]),
    u256([6, 0, 0, 0]),
    u256([324, 0, 0, 0]),
    u256([7, 0, 0, 0]),
    u256([409, 0, 0, 0]),
    u256([302, 0, 0, 0]),
    u256([260, 0, 0, 0]),
    u256([216, 0, 0, 0]),
    u256([320, 0, 0, 0]),
    u256([166, 0, 0, 0]),
    u256([475, 0, 0, 0]),
    u256([465, 0, 0, 0]),
    u256([45, 0, 0, 0]),
    u256([366, 0, 0, 0]),
    u256([519, 0, 0, 0]),
    u256([335, 0, 0, 0]),
    u256([200, 0, 0, 0]),
    u256([215, 0, 0, 0]),
    u256([205, 0, 0, 0]),
    u256([262, 0, 0, 0]),
    u256([419, 0, 0, 0]),
    u256([147, 0, 0, 0]),
    u256([237, 0, 0, 0]),
    u256([282, 0, 0, 0]),
    u256([359, 0, 0, 0]),
    u256([174, 0, 0, 0]),
    u256([379, 0, 0, 0]),
    u256([441, 0, 0, 0]),
    u256([551, 0, 0, 0]),
    u256([473, 0, 0, 0]),
    u256([605, 0, 0, 0]),
    u256([427, 0, 0, 0]),
    u256([474, 0, 0, 0]),
    u256([387, 0, 0, 0]),
    u256([84, 0, 0, 0]),
    u256([171, 0, 0, 0]),
    u256([222, 0, 0, 0]),
    u256([37, 0, 0, 0]),
    u256([565, 0, 0, 0]),
    u256([48, 0, 0, 0]),
    u256([549, 0, 0, 0]),
    u256([161, 0, 0, 0]),
    u256([521, 0, 0, 0]),
    u256([566, 0, 0, 0]),
    u256([518, 0, 0, 0]),
    u256([568, 0, 0, 0]),
    u256([403, 0, 0, 0]),
    u256([597, 0, 0, 0]),
    u256([397, 0, 0, 0]),
    u256([154, 0, 0, 0]),
    u256([649, 0, 0, 0]),
    u256([53, 0, 0, 0]),
    u256([522, 0, 0, 0]),
    u256([416, 0, 0, 0]),
    u256([240, 0, 0, 0]),
    u256([372, 0, 0, 0]),
    u256([645, 0, 0, 0]),
    u256([261, 0, 0, 0]),
    u256([314, 0, 0, 0]),
    u256([309, 0, 0, 0]),
    u256([395, 0, 0, 0]),
    u256([373, 0, 0, 0]),
    u256([20, 0, 0, 0]),
    u256([119, 0, 0, 0]),
    u256([27, 0, 0, 0]),
    u256([608, 0, 0, 0]),
    u256([340, 0, 0, 0]),
    u256([609, 0, 0, 0]),
    u256([361, 0, 0, 0]),
    u256([503, 0, 0, 0]),
    u256([241, 0, 0, 0]),
    u256([602, 0, 0, 0]),
    u256([30, 0, 0, 0]),
    u256([275, 0, 0, 0]),
    u256([569, 0, 0, 0]),
    u256([423, 0, 0, 0]),
    u256([454, 0, 0, 0]),
    u256([150, 0, 0, 0]),
    u256([621, 0, 0, 0]),
    u256([415, 0, 0, 0]),
    u256([344, 0, 0, 0]),
    u256([535, 0, 0, 0]),
    u256([411, 0, 0, 0]),
    u256([540, 0, 0, 0]),
    u256([199, 0, 0, 0]),
    u256([442, 0, 0, 0]),
    u256([371, 0, 0, 0]),
    u256([404, 0, 0, 0]),
    u256([210, 0, 0, 0]),
    u256([322, 0, 0, 0]),
    u256([432, 0, 0, 0]),
    u256([492, 0, 0, 0]),
    u256([560, 0, 0, 0]),
    u256([250, 0, 0, 0]),
    u256([132, 0, 0, 0]),
    u256([627, 0, 0, 0]),
    u256([233, 0, 0, 0]),
    u256([202, 0, 0, 0]),
    u256([304, 0, 0, 0]),
    u256([641, 0, 0, 0]),
    u256([338, 0, 0, 0]),
    u256([74, 0, 0, 0]),
    u256([575, 0, 0, 0]),
    u256([408, 0, 0, 0]),
    u256([425, 0, 0, 0]),
    u256([291, 0, 0, 0]),
    u256([135, 0, 0, 0]),
    u256([318, 0, 0, 0]),
    u256([601, 0, 0, 0]),
    u256([159, 0, 0, 0]),
    u256([489, 0, 0, 0]),
    u256([556, 0, 0, 0]),
    u256([385, 0, 0, 0]),
    u256([548, 0, 0, 0]),
    u256([554, 0, 0, 0]),
    u256([81, 0, 0, 0]),
    u256([362, 0, 0, 0]),
    u256([108, 0, 0, 0]),
    u256([270, 0, 0, 0]),
    u256([405, 0, 0, 0]),
    u256([136, 0, 0, 0]),
    u256([576, 0, 0, 0]),
    u256([55, 0, 0, 0]),
    u256([389, 0, 0, 0]),
    u256([354, 0, 0, 0]),
    u256([604, 0, 0, 0]),
    u256([388, 0, 0, 0]),
    u256([97, 0, 0, 0]),
    u256([198, 0, 0, 0]),
    u256([317, 0, 0, 0]),
    u256([334, 0, 0, 0]),
    u256([458, 0, 0, 0]),
    u256([491, 0, 0, 0]),
    u256([259, 0, 0, 0]),
    u256([583, 0, 0, 0]),
    u256([369, 0, 0, 0]),
    u256([129, 0, 0, 0]),
    u256([546, 0, 0, 0]),
    u256([87, 0, 0, 0]),
    u256([327, 0, 0, 0]),
    u256([266, 0, 0, 0]),
    u256([401, 0, 0, 0]),
    u256([550, 0, 0, 0]),
    u256([69, 0, 0, 0]),
    u256([274, 0, 0, 0]),
    u256([615, 0, 0, 0]),
    u256([400, 0, 0, 0]),
    u256([181, 0, 0, 0]),
    u256([353, 0, 0, 0]),
    u256([196, 0, 0, 0]),
    u256([456, 0, 0, 0]),
    u256([595, 0, 0, 0]),
    u256([420, 0, 0, 0]),
    u256([122, 0, 0, 0]),
    u256([392, 0, 0, 0]),
    u256([185, 0, 0, 0]),
    u256([516, 0, 0, 0]),
    u256([466, 0, 0, 0]),
    u256([476, 0, 0, 0]),
    u256([75, 0, 0, 0]),
    u256([235, 0, 0, 0]),
    u256([530, 0, 0, 0]),
    u256([448, 0, 0, 0]),
    u256([594, 0, 0, 0]),
    u256([378, 0, 0, 0]),
    u256([455, 0, 0, 0]),
    u256([447, 0, 0, 0]),
    u256([577, 0, 0, 0]),
    u256([285, 0, 0, 0]),
    u256([99, 0, 0, 0]),
    u256([558, 0, 0, 0]),
    u256([653, 0, 0, 0]),
    u256([410, 0, 0, 0]),
    u256([461, 0, 0, 0]),
    u256([160, 0, 0, 0]),
    u256([331, 0, 0, 0]),
    u256([290, 0, 0, 0]),
    u256([563, 0, 0, 0]),
    u256([613, 0, 0, 0]),
    u256([219, 0, 0, 0]),
    u256([394, 0, 0, 0]),
    u256([29, 0, 0, 0]),
    u256([552, 0, 0, 0]),
    u256([9, 0, 0, 0]),
    u256([189, 0, 0, 0]),
    u256([298, 0, 0, 0]),
    u256([137, 0, 0, 0]),
    u256([56, 0, 0, 0]),
    u256([636, 0, 0, 0]),
    u256([12, 0, 0, 0]),
    u256([581, 0, 0, 0]),
    u256([2, 0, 0, 0]),
    u256([109, 0, 0, 0]),
    u256([339, 0, 0, 0]),
    u256([127, 0, 0, 0]),
    u256([36, 0, 0, 0]),
    u256([443, 0, 0, 0]),
    u256([573, 0, 0, 0]),
    u256([523, 0, 0, 0]),
    u256([451, 0, 0, 0]),
    u256([479, 0, 0, 0]),
    u256([286, 0, 0, 0]),
    u256([28, 0, 0, 0]),
    u256([116, 0, 0, 0]),
    u256([312, 0, 0, 0]),
    u256([628, 0, 0, 0]),
    u256([545, 0, 0, 0]),
    u256([54, 0, 0, 0]),
    u256([82, 0, 0, 0]),
    u256([651, 0, 0, 0]),
    u256([482, 0, 0, 0]),
    u256([131, 0, 0, 0]),
    u256([26, 0, 0, 0]),
    u256([396, 0, 0, 0]),
    u256([271, 0, 0, 0]),
    u256([593, 0, 0, 0]),
    u256([124, 0, 0, 0]),
    u256([107, 0, 0, 0]),
    u256([515, 0, 0, 0]),
    u256([114, 0, 0, 0]),
    u256([407, 0, 0, 0]),
    u256([654, 0, 0, 0]),
    u256([268, 0, 0, 0]),
    u256([342, 0, 0, 0]),
    u256([277, 0, 0, 0]),
    u256([254, 0, 0, 0]),
    u256([14, 0, 0, 0]),
    u256([79, 0, 0, 0]),
    u256([191, 0, 0, 0]),
    u256([43, 0, 0, 0]),
    u256([252, 0, 0, 0]),
    u256([512, 0, 0, 0]),
    u256([256, 0, 0, 0]),
    u256([220, 0, 0, 0]),
    u256([381, 0, 0, 0]),
    u256([66, 0, 0, 0]),
    u256([481, 0, 0, 0]),
    u256([19, 0, 0, 0]),
    u256([228, 0, 0, 0]),
    u256([367, 0, 0, 0]),
    u256([487, 0, 0, 0]),
    u256([434, 0, 0, 0]),
    u256([349, 0, 0, 0]),
    u256([144, 0, 0, 0]),
    u256([460, 0, 0, 0]),
    u256([91, 0, 0, 0]),
    u256([495, 0, 0, 0]),
    u256([78, 0, 0, 0]),
    u256([195, 0, 0, 0]),
    u256([490, 0, 0, 0]),
    u256([67, 0, 0, 0]),
    u256([486, 0, 0, 0]),
    u256([64, 0, 0, 0]),
    u256([105, 0, 0, 0]),
    u256([467, 0, 0, 0]),
    u256([231, 0, 0, 0]),
    u256([507, 0, 0, 0]),
    u256([376, 0, 0, 0]),
    u256([248, 0, 0, 0]),
    u256([631, 0, 0, 0]),
    u256([520, 0, 0, 0]),
    u256([464, 0, 0, 0]),
    u256([221, 0, 0, 0]),
    u256([433, 0, 0, 0]),
    u256([622, 0, 0, 0]),
    u256([531, 0, 0, 0]),
    u256([197, 0, 0, 0]),
    u256([61, 0, 0, 0]),
    u256([163, 0, 0, 0]),
    u256([98, 0, 0, 0]),
    u256([648, 0, 0, 0]),
    u256([146, 0, 0, 0]),
    u256([238, 0, 0, 0]),
    u256([494, 0, 0, 0]),
    u256([125, 0, 0, 0]),
    u256([76, 0, 0, 0]),
    u256([242, 0, 0, 0]),
    u256([463, 0, 0, 0]),
    u256([326, 0, 0, 0]),
    u256([38, 0, 0, 0]),
    u256([152, 0, 0, 0]),
    u256([438, 0, 0, 0]),
    u256([345, 0, 0, 0]),
    u256([637, 0, 0, 0]),
    u256([40, 0, 0, 0]),
    u256([17, 0, 0, 0]),
    u256([281, 0, 0, 0]),
    u256([323, 0, 0, 0]),
    u256([110, 0, 0, 0]),
    u256([118, 0, 0, 0]),
    u256([578, 0, 0, 0]),
    u256([139, 0, 0, 0]),
    u256([315, 0, 0, 0]),
    u256([115, 0, 0, 0]),
    u256([62, 0, 0, 0]),
    u256([470, 0, 0, 0]),
    u256([293, 0, 0, 0]),
    u256([265, 0, 0, 0]),
    u256([258, 0, 0, 0]),
    u256([553, 0, 0, 0]),
    u256([301, 0, 0, 0]),
    u256([610, 0, 0, 0]),
    u256([555, 0, 0, 0]),
    u256([305, 0, 0, 0]),
    u256([634, 0, 0, 0]),
    u256([308, 0, 0, 0]),
    u256([626, 0, 0, 0]),
    u256([180, 0, 0, 0]),
    u256([253, 0, 0, 0]),
    u256([350, 0, 0, 0]),
    u256([502, 0, 0, 0]),
    u256([184, 0, 0, 0]),
    u256([431, 0, 0, 0]),
    u256([294, 0, 0, 0]),
    u256([264, 0, 0, 0]),
    u256([288, 0, 0, 0]),
    u256([632, 0, 0, 0]),
    u256([25, 0, 0, 0]),
    u256([607, 0, 0, 0]),
    u256([485, 0, 0, 0]),
    u256([592, 0, 0, 0]),
    u256([263, 0, 0, 0]),
    u256([128, 0, 0, 0]),
    u256([547, 0, 0, 0]),
    u256([100, 0, 0, 0]),
    u256([346, 0, 0, 0]),
];

/// Decomposition of -1 = [v_n, v_{n-1} ..., v_1], i.e. the representation of q-1
pub const BLS_SCALAR_REAL: [u256; 27] = [
    u256([660, 0, 0, 0]),
    u256([660, 0, 0, 0]),
    u256([673, 0, 0, 0]),
    u256([663, 0, 0, 0]),
    u256([674, 0, 0, 0]),
    u256([682, 0, 0, 0]),
    u256([687, 0, 0, 0]),
    u256([683, 0, 0, 0]),
    u256([669, 0, 0, 0]),
    u256([684, 0, 0, 0]),
    u256([672, 0, 0, 0]),
    u256([666, 0, 0, 0]),
    u256([680, 0, 0, 0]),
    u256([662, 0, 0, 0]),
    u256([686, 0, 0, 0]),
    u256([668, 0, 0, 0]),
    u256([661, 0, 0, 0]),
    u256([678, 0, 0, 0]),
    u256([692, 0, 0, 0]),
    u256([686, 0, 0, 0]),
    u256([689, 0, 0, 0]),
    u256([660, 0, 0, 0]),
    u256([690, 0, 0, 0]),
    u256([687, 0, 0, 0]),
    u256([683, 0, 0, 0]),
    u256([674, 0, 0, 0]),
    u256([678, 0, 0, 0]),
];

/// decomposition = [s_n, s_{n-1} ..., s_1]
pub const DECOMPOSITION_S_I: [BlsScalar; 27] = [
    BlsScalar([693, 0, 0, 0]),
    BlsScalar([696, 0, 0, 0]),
    BlsScalar([694, 0, 0, 0]),
    BlsScalar([668, 0, 0, 0]),
    BlsScalar([679, 0, 0, 0]),
    BlsScalar([695, 0, 0, 0]),
    BlsScalar([691, 0, 0, 0]),
    BlsScalar([693, 0, 0, 0]),
    BlsScalar([700, 0, 0, 0]),
    BlsScalar([688, 0, 0, 0]),
    BlsScalar([700, 0, 0, 0]),
    BlsScalar([694, 0, 0, 0]),
    BlsScalar([701, 0, 0, 0]),
    BlsScalar([694, 0, 0, 0]),
    BlsScalar([699, 0, 0, 0]),
    BlsScalar([701, 0, 0, 0]),
    BlsScalar([701, 0, 0, 0]),
    BlsScalar([701, 0, 0, 0]),
    BlsScalar([695, 0, 0, 0]),
    BlsScalar([698, 0, 0, 0]),
    BlsScalar([697, 0, 0, 0]),
    BlsScalar([703, 0, 0, 0]),
    BlsScalar([702, 0, 0, 0]),
    BlsScalar([691, 0, 0, 0]),
    BlsScalar([688, 0, 0, 0]),
    BlsScalar([703, 0, 0, 0]),
    BlsScalar([679, 0, 0, 0]),
];

/// decomposition_inverses (are in Montgomery form) = [s_n^{-1}, ...,
/// s_1^{-1}]
pub const INVERSES_S_I: [BlsScalar; 27] = [
    BlsScalar([
        10221572469640980478,
        9996602938199176322,
        12629687422955549043,
        8006457294428823986,
    ]),
    BlsScalar([
        2544378491691269391,
        4278332345953639965,
        11874303680749211767,
        2426939821791091123,
    ]),
    BlsScalar([
        744249036390430597,
        12703574479313007857,
        12941063472551889101,
        7898626469976166322,
    ]),
    BlsScalar([
        12813307260754568983,
        13401737678607148669,
        16259656273855951252,
        77635945376597827,
    ]),
    BlsScalar([
        5868183680725469139,
        13963385237577580741,
        2771541177262918091,
        2057109021922450213,
    ]),
    BlsScalar([
        12076645397819930386,
        3513102646780106621,
        5511421539288173570,
        5771838565320455168,
    ]),
    BlsScalar([
        6887496337358022873,
        10595820730184048433,
        5009316404333812521,
        5382134171948568183,
    ]),
    BlsScalar([
        10221572469640980478,
        9996602938199176322,
        12629687422955549043,
        8006457294428823986,
    ]),
    BlsScalar([
        4216398644020393879,
        8184599923631624209,
        8370160424198379966,
        2747212268616577148,
    ]),
    BlsScalar([
        9008875010444565957,
        5372543878254038396,
        4977331236853060682,
        4494972308657887955,
    ]),
    BlsScalar([
        4216398644020393879,
        8184599923631624209,
        8370160424198379966,
        2747212268616577148,
    ]),
    BlsScalar([
        744249036390430597,
        12703574479313007857,
        12941063472551889101,
        7898626469976166322,
    ]),
    BlsScalar([
        14604768842820473715,
        14849596265064785286,
        7748405401081812358,
        5829685384640365743,
    ]),
    BlsScalar([
        744249036390430597,
        12703574479313007857,
        12941063472551889101,
        7898626469976166322,
    ]),
    BlsScalar([
        158341149692136564,
        11915096076612138202,
        992774181608407826,
        2273115756299035808,
    ]),
    BlsScalar([
        14604768842820473715,
        14849596265064785286,
        7748405401081812358,
        5829685384640365743,
    ]),
    BlsScalar([
        14604768842820473715,
        14849596265064785286,
        7748405401081812358,
        5829685384640365743,
    ]),
    BlsScalar([
        14604768842820473715,
        14849596265064785286,
        7748405401081812358,
        5829685384640365743,
    ]),
    BlsScalar([
        12076645397819930386,
        3513102646780106621,
        5511421539288173570,
        5771838565320455168,
    ]),
    BlsScalar([
        14693968055083782227,
        11284659261154016671,
        1375836735433574310,
        7087423626300479469,
    ]),
    BlsScalar([
        8098570567349679003,
        8435273170816927395,
        2562595664368193229,
        3130570460078912387,
    ]),
    BlsScalar([
        9026571779888472938,
        4361001158901360622,
        8564805700575463153,
        287658769533324946,
    ]),
    BlsScalar([
        15293454485951598931,
        15506724579931865072,
        2628833535283587331,
        4905112046230959809,
    ]),
    BlsScalar([
        6887496337358022873,
        10595820730184048433,
        5009316404333812521,
        5382134171948568183,
    ]),
    BlsScalar([
        9008875010444565957,
        5372543878254038396,
        4977331236853060682,
        4494972308657887955,
    ]),
    BlsScalar([
        9026571779888472938,
        4361001158901360622,
        8564805700575463153,
        287658769533324946,
    ]),
    BlsScalar([
        5868183680725469139,
        13963385237577580741,
        2771541177262918091,
        2057109021922450213,
    ]),
];