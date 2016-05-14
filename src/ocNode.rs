use point::{Point};
use pointCloud::{PointCloud};
use functions::{center, calc_sub_min_max, calc_direction, in_bb};
//@todo either merge Oct code or split KdNode and Tree into seperate files

pub enum OcNode {
    Leaf(Point),
    Node(Internal)
}

struct Internal { // naming : p == positive, n == negative ||| xyz   => pnp => x positive, y negative, z positive direction from center
    ppp: Option<Box<OcNode>>,
    ppn: Option<Box<OcNode>>,
    pnp: Option<Box<OcNode>>,
    pnn: Option<Box<OcNode>>,
    npp: Option<Box<OcNode>>,
    npn: Option<Box<OcNode>>,
    nnp: Option<Box<OcNode>>,
    nnn: Option<Box<OcNode>>
}

pub enum Direction { //@todo rename //@todo private?
    PPP,
    PPN,
    PNP,
    PNN,
    NPP,
    NPN,
    NNP,
    NNN
}

//@todo define somewhere else
fn collect_center_or_all(n: &OcNode, onlyCollectCenters: bool, depth: i8, maxdepth: i8, mut pc: &mut PointCloud) {
    if onlyCollectCenters {
        let mut subPc = PointCloud::new();
        n.collect(depth+1, maxdepth, &mut subPc);
        if let Some(c) = subPc.center() {
            pc.push(c);
        }
    } else {
        n.collect(depth+1, maxdepth, pc);
    }
}

///@todo define somewhere else
fn build_subnode(pc: Vec<Point>,bb: (Point, Point)) -> Option<Box<OcNode>> {
    match pc.len() {
        0 => None,
        _ => {
            let (newMin, newMax) = bb;
            Some(Box::new(OcNode::new(&newMin, &newMax, pc)))
        }
    }
}


impl OcNode {
    pub fn new(min: &Point, max: &Point, mut pc: Vec<Point>) -> OcNode {
        if pc.len() == 1 { return OcNode::Leaf(pc[0].clone()); };
        let mut pcppp = Vec::new();
        let mut pcppn = Vec::new();
        let mut pcpnp = Vec::new();
        let mut pcpnn = Vec::new();
        let mut pcnpp = Vec::new();
        let mut pcnpn = Vec::new();
        let mut pcnnp = Vec::new();
        let mut pcnnn = Vec::new();

        let bbppp = calc_sub_min_max(Direction::PPP, min, max);
        let bbppn = calc_sub_min_max(Direction::PPN, min, max);
        let bbpnp = calc_sub_min_max(Direction::PNP, min, max);
        let bbpnn = calc_sub_min_max(Direction::PNN, min, max);
        let bbnpp = calc_sub_min_max(Direction::NPP, min, max);
        let bbnpn = calc_sub_min_max(Direction::NPN, min, max);
        let bbnnp = calc_sub_min_max(Direction::NNP, min, max);
        let bbnnn = calc_sub_min_max(Direction::NNN, min, max);

        for p in pc {
            if in_bb(&p, &bbppp.0, &bbppp.1) {
                pcppp.push(p);
            } else if in_bb(&p, &bbppn.0, &bbppn.1) {
                pcppn.push(p);
            } else if in_bb(&p, &bbpnp.0, &bbpnp.1) {
                pcpnp.push(p);
            } else if in_bb(&p, &bbpnn.0, &bbpnn.1) {
                pcpnn.push(p);
            } else if in_bb(&p, &bbnpp.0, &bbnpp.1) {
                pcnpp.push(p);
            } else if in_bb(&p, &bbnpn.0, &bbnpn.1) {
                pcnpn.push(p);
            } else if in_bb(&p, &bbnnp.0, &bbnnp.1) {
                pcnnp.push(p);
            } else if in_bb(&p, &bbnnn.0, &bbnnn.1) {
                pcnnn.push(p);
            }
        }

        let ppp = build_subnode(pcppp, bbppp);
        let ppn = build_subnode(pcppn, bbppn);
        let pnp = build_subnode(pcpnp, bbpnp);
        let pnn = build_subnode(pcpnn, bbpnn);
        let npp = build_subnode(pcnpp, bbnpp);
        let npn = build_subnode(pcnpn, bbnpn);
        let nnp = build_subnode(pcnnp, bbnnp);
        let nnn = build_subnode(pcnnn, bbnnn);

        let mut result: Internal = Internal {
            ppp: ppp,
            ppn: ppn,
            pnp: pnp,
            pnn: pnn,
            npp: npp,
            npn: npn,
            nnp: nnp,
            nnn: nnn
        };

        OcNode::Node(result)
    }

//@todo define helpers here to simplify code (and in other areas)
    pub fn collect(&self, depth: i8, maxdepth: i8, pc: &mut PointCloud) {
        let onlyCollectCenters = maxdepth >= 0 && depth > maxdepth; //@todo make this depend on a setting?
        match self {
            &OcNode::Leaf(ref p) => pc.push(p.clone()),

            &OcNode::Node(ref internal) => {
                if let Some(ref n) = internal.ppp {
                    collect_center_or_all(n, onlyCollectCenters, depth, maxdepth, pc);
                }
                if let Some(ref n) = internal.ppn {
                    collect_center_or_all(n, onlyCollectCenters, depth, maxdepth, pc);
                }
                if let Some(ref n) = internal.pnp {
                    collect_center_or_all(n, onlyCollectCenters, depth, maxdepth, pc);
                }
                if let Some(ref n) = internal.pnn {
                    collect_center_or_all(n, onlyCollectCenters, depth, maxdepth, pc);
                }
                if let Some(ref n) = internal.npp {
                    collect_center_or_all(n, onlyCollectCenters, depth, maxdepth, pc);
                }
                if let Some(ref n) = internal.npn {
                    collect_center_or_all(n, onlyCollectCenters, depth, maxdepth, pc);
                }
                if let Some(ref n) = internal.nnp {
                    collect_center_or_all(n, onlyCollectCenters, depth, maxdepth, pc);
                }
                if let Some(ref n) = internal.nnn {
                    collect_center_or_all(n, onlyCollectCenters, depth, maxdepth, pc);
                }
            }
        }
    }
}
