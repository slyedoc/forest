use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

type V = [f32; 3];

#[derive(Inspectable, Clone)]
pub struct TreeProperties {
    clumpMax: f32,
    clumpMin: f32,
    lengthFalloffFactor: f32,
    lengthFalloffPower: usize,
    branchFactor: f32,
    radiusFalloffRate: f32,
    climbRate: f32,
    trunkKink: f32,
    maxRadius: f32,
    treeSteps: usize,
    taperRate: f32,
    twistRate: usize,
    segments: usize,
    levels: usize,
    sweepAmount: f32,
    initalBranchLength: f32,
    trunkLength: f32,
    dropAmount: f32,
    growAmount: f32,
    vMultiplier: f32,
    twigScale: f32,
    seed: usize,
    rseed: usize,
}


#[derive(PartialEq, Clone)]
pub struct Branch {
    child0: Option<Entity>,
    child1: Option<Entity>,
    parent: Option<Entity>,
    head: Option<Entity>,

    length: f32,
    radius: f32,
    tangent: V,
    root: Vec<usize>
}

impl Branch {
    fn new( head: Option<Entity>, parent: Option<Entity>) -> Self {
        Self {
            head: head,
            parent: parent,
            radius: 1.0,
            length: Default::default(),
            child0: None,
            child1: None,
            tangent: [0.0, 0.0, 0.0],
            root: Vec::new(),
        }
    }
}

pub struct Tree {
    // Display
    properties: TreeProperties,
    root: Branch,
    verts: Vec<V>,
    faces: Vec<[usize; 3]>,
    normals: Vec<V>,
    UV: Vec<[f32; 2]>,
    vertsTwig: Vec<V>,
    normalsTwig: Vec<V>,
    facesTwig: Vec<f32>,
    uvsTwig: Vec<f32>,
}

impl Tree {
    pub fn new(data: &TreeProperties) -> Self {
        let mut props = data.clone();
        props.rseed = data.seed;
        let mut tree = Self {
            properties: props,
            root: Branch::new(None, None),
            verts: Vec::new(),
            faces: Vec::new(),
            normals: Vec::new(),
            UV: Vec::new(),
            vertsTwig: Vec::new(),
            normalsTwig: Vec::new(),
            facesTwig: Vec::new(),
            uvsTwig: Vec::new(),
        };

        //tree.root.split(null, null, this.properties);
        tree.createForks( None, None);
        //tree.createTwigs();
        //tree.doFaces();
        tree.calcNormals();

        tree
    }
    pub fn random(&mut self, mut seed: usize) -> f32 {
        if !seed == 0 {
            seed = self.properties.rseed;
            self.properties.rseed += 1;
        };
        return (seed as f32 + seed as f32 * seed as f32).cos().abs();
    }

    fn calcNormals(&mut self) {
        let mut allNormals = Vec::new();
        for i in 0..self.verts.len() {
            allNormals.push(Vec::new());
        }
        for i in 0..self.faces.len() {
            let face = self.faces[i];
            let norm = normalize(cross(
                subVec(self.verts[face[1]], self.verts[face[2]]),
                subVec(self.verts[face[1]], self.verts[face[0]]),
            ));
            allNormals[face[0]].push(norm);
            allNormals[face[1]].push(norm);
            allNormals[face[2]].push(norm);
        }
        for i in 0..allNormals.len() {
            let mut total = [0.0, 0.0, 0.0];
            let l = allNormals[i].len();
            for j in 0..l {
                total = addVec(total, scaleVec(allNormals[i][j], 1.0 / l as f32));
            }
            self.normals[i] = total;
        }
    }

    fn createForks(&mut self, branch: Option<Branch>, radius: Option<f32>) {
        let mut branch = match branch {
            Some(b) => b,
            None => self.root.clone(),
        };

        let mut radius = match radius {
            Some(r) => r,
            None => self.properties.maxRadius,
        };

        branch.radius = radius;

        if radius > branch.length {
            radius=branch.length;
        }

        let segmentAngle= std::f32::consts::PI * 2.0 / self.properties.segments as f32;
        if branch.parent == None {
          //create the root of the tree


          let axis=[0.0,1.0,0.0];
          for i in 0..self.properties.segments {
            let vec=vecAxisAngle([-1.0,0.0,0.0], axis,-segmentAngle * i as f32);
            branch.root.push(self.verts.len());
            self.verts.push(scaleVec(vec,radius / self.properties.radiusFalloffRate));
          }
        }

    //     //cross the branches to get the left
    //     //add the branches to get the up
    //     if let Some(child) = branch.child0 {
    //       let axis = match branch.parent {
    //         Some(b) => normalize(subVec(branch.head.unwrap(),branch.parent.head));,
    //         None => normalize(branch.head),
    //       };
  
    //       let axis1= normalize(subVec(branch.head.unwrap(),branch.child0.unwrap().head.unwrap()));
    //       let axis2= normalize(subVec(branch.head.unwrap(),branch.child1.unwrap().head.unwrap()));
    //       let tangent=normalize(cross(axis1,axis2));
    //       branch.tangent=tangent;
      
    //       let axis3=normalize(cross(tangent,normalize(addVec(scaleVec(axis1,-1),scaleVec(axis2,-1)))));
    //       let dir=[axis2[0],0.0,axis2[2]];
    //       let centerloc=addVec(branch.head,scaleVec(dir,-this.properties.maxRadius/2));
      
      
      
    //       var ring0=branch.ring0=[];
    //       var ring1=branch.ring1=[];
    //       var ring2=branch.ring2=[];
      
    //       var scale=this.properties.radiusFalloffRate;
      
    //       if(branch.child0.type=="trunk" || branch.type=="trunk") {
    //         scale=1/this.properties.taperRate;
    //       }
      
    //       //main segment ring
    //       var linch0=verts.length;
    //       ring0.push(linch0);
    //       ring2.push(linch0);
    //       verts.push(addVec(centerloc,scaleVec(tangent,radius*scale)));
      
    //       var start=verts.length-1;
    //       var d1=vecAxisAngle(tangent,axis2,1.57);
    //       var d2=normalize(cross(tangent,axis));
    //       var s=1/dot(d1,d2);
    //       for(var i=1;i<segments/2;i++){
    //         var vec=vecAxisAngle(tangent,axis2,segmentAngle*i);
    //         ring0.push(start+i);
    //         ring2.push(start+i);
    //         vec=scaleInDirection(vec,d2,s);
    //         verts.push(addVec(centerloc,scaleVec(vec,radius*scale)));
    //       }
    //       var linch1=verts.length;
    //       ring0.push(linch1);
    //       ring1.push(linch1);
    //       verts.push(addVec(centerloc,scaleVec(tangent,-radius*scale)));
    //       for(var i=segments/2+1;i<segments;i++){
    //         var vec=vecAxisAngle(tangent,axis1,segmentAngle*i);
    //         ring0.push(verts.length);
    //         ring1.push(verts.length);
    //         verts.push(addVec(centerloc,scaleVec(vec,radius*scale)));
      
    //       }
    //       ring1.push(linch0);
    //       ring2.push(linch1);
    //       var start=verts.length-1;
    //       for(var i=1;i<segments/2;i++){
    //         var vec=vecAxisAngle(tangent,axis3,segmentAngle*i);
    //         ring1.push(start+i);
    //         ring2.push(start+(segments/2-i));
    //         var v=scaleVec(vec,radius*scale);
    //         verts.push(addVec(centerloc,v));
    //       }
      
    //       //child radius is related to the brans direction and the length of the branch
    //       var length0=length(subVec(branch.head,branch.child0.head));
    //       var length1=length(subVec(branch.head,branch.child1.head));
      
    //       var radius0=1*radius*this.properties.radiusFalloffRate;
    //       var radius1=1*radius*this.properties.radiusFalloffRate;
    //       if(branch.child0.type=="trunk") radius0=radius*this.properties.taperRate;
    //       this.createForks(branch.child0,radius0);
    //       this.createForks(branch.child1,radius1);
    //     }else{
    //       //add points for the ends of braches
    //       branch.end=verts.length;
    //       //branch.head=addVec(branch.head,scaleVec([this.properties.xBias,this.properties.yBias,this.properties.zBias],branch.length*3));
    //       verts.push(branch.head);
      
    //     }
      
     }
}


fn dot(v1: V, v2: V) -> f32 {
    v1[0] * v2[0] + v1[1] * v2[1] + v1[2] * v2[2]
}
fn cross(v1: V, v2: V) -> V {
    [
        v1[1] * v2[2] - v1[2] * v2[1],
        v1[2] * v2[0] - v1[0] * v2[2],
        v1[0] * v2[1] - v1[1] * v2[0],
    ]
}

fn length(v: V) -> f32 {
    (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt()
}
fn normalize(v: V) -> V {
    let l = length(v);
    scaleVec(v, 1.0 / l)
}
fn scaleVec(v: V, s: f32) -> V {
    [v[0] * s, v[1] * s, v[2] * s]
}
fn subVec(v1: V, v2: V) -> V {
    [v1[0] - v2[0], v1[1] - v2[1], v1[2] - v2[2]]
}
fn addVec(v1: V, v2: V) -> V {
    [v1[0] + v2[0], v1[1] + v2[1], v1[2] + v2[2]]
}

fn vecAxisAngle(vec: V, axis: V, angle: f32) -> V {
    //v cos(T) + (axis x v) * sin(T) + axis*(axis . v)(1-cos(T)
    let cosr = angle.cos();
    let sinr = angle.sin();
    addVec(
        addVec(scaleVec(vec, cosr), scaleVec(cross(axis, vec), sinr)),
        scaleVec(axis, dot(axis, vec) * (1.0 - cosr)),
    )
}

fn scaleInDirection(vector: V, direction: V, scale: f32) -> V {
    let currentMag = dot(vector, direction);
    let change = scaleVec(direction, currentMag * scale - currentMag);
    addVec(vector, change)
}

//   Tree.prototype.

//   Tree.prototype.doFaces=function(branch){
//     if(!branch) branch=this.root;
//     let mutsegments=this.properties.segments;
//     let mutfaces=this.faces;
//     let mutverts=this.verts;
//     let mutUV=this.UV;
//     if(!branch.parent){
//       for(i=0;i<verts.length;i++){
//         UV[i]=[0,0];
//       }
//       let muttangent=normalize(cross(subVec(branch.child0.head,branch.head),subVec(branch.child1.head,branch.head)));
//       let mutnormal=normalize(branch.head);
//       let mutangle=Math.acos(dot(tangent,[-1,0,0]));
//       if(dot(cross([-1,0,0],tangent),normal)>0) angle=2*Math.PI-angle;
//       let mutsegOffset=Math.round((angle/Math.PI/2*segments));
//       for(let muti=0;i<segments;i++){
//         let mutv1=branch.ring0[i];
//         let mutv2=branch.root[(i+segOffset+1)%segments];
//         let mutv3=branch.root[(i+segOffset)%segments];
//         let mutv4=branch.ring0[(i+1)%segments];

//         faces.push([v1,v4,v3]);
//         faces.push([v4,v2,v3]);
//         UV[(i+segOffset)%segments]=[Math.abs(i/segments-0.5)*2,0];
//         let mutlen=length(subVec(verts[branch.ring0[i]],verts[branch.root[(i+segOffset)%segments]]))*this.properties.vMultiplier;
//         UV[branch.ring0[i]]=[Math.abs(i/segments-0.5)*2,len];
//         UV[branch.ring2[i]]=[Math.abs(i/segments-0.5)*2,len];
//       }
//     }

//     if(branch.child0.ring0){
//       let mutsegOffset0,segOffset1;
//       let mutmatch0,match1;

//       let mutv1=normalize(subVec(verts[branch.ring1[0]],branch.head));
//       let mutv2=normalize(subVec(verts[branch.ring2[0]],branch.head));

//       v1=scaleInDirection(v1,normalize(subVec(branch.child0.head,branch.head)),0);
//       v2=scaleInDirection(v2,normalize(subVec(branch.child1.head,branch.head)),0);

//       for(i=0;i<segments;i++){
//         let mutd=normalize(subVec(verts[branch.child0.ring0[i]],branch.child0.head));
//         let mutl=dot(d,v1);
//         if(segOffset0===undefined || l>match0){
//           match0=l;
//           segOffset0=segments-i;
//         }
//         d=normalize(subVec(verts[branch.child1.ring0[i]],branch.child1.head));
//         l=dot(d,v2);
//         if(segOffset1==undefined || l>match1){
//           match1=l;
//           segOffset1=segments-i;
//         }
//       }

//       let mutUVScale=this.properties.maxRadius/branch.radius;

//       for(i=0;i<segments;i++){
//         v1=branch.child0.ring0[i];
//         v2=branch.ring1[(i+segOffset0+1)%segments];
//         v3=branch.ring1[(i+segOffset0)%segments];
//         v4=branch.child0.ring0[(i+1)%segments];
//         faces.push([v1,v4,v3]);
//         faces.push([v4,v2,v3]);
//         v1=branch.child1.ring0[i];
//         v2=branch.ring2[(i+segOffset1+1)%segments];
//         v3=branch.ring2[(i+segOffset1)%segments];
//         v4=branch.child1.ring0[(i+1)%segments];
//         faces.push([v1,v2,v3]);
//         faces.push([v1,v4,v2]);

//         let mutlen1=length(subVec(verts[branch.child0.ring0[i]],verts[branch.ring1[(i+segOffset0)%segments]]))*UVScale;
//         let mutuv1=UV[branch.ring1[(i+segOffset0-1)%segments]];

//         UV[branch.child0.ring0[i]]=[uv1[0],uv1[1]+len1*this.properties.vMultiplier];
//         UV[branch.child0.ring2[i]]=[uv1[0],uv1[1]+len1*this.properties.vMultiplier];

//         let mutlen2=length(subVec(verts[branch.child1.ring0[i]],verts[branch.ring2[(i+segOffset1)%segments]]))*UVScale;
//         let mutuv2=UV[branch.ring2[(i+segOffset1-1)%segments]];

//         UV[branch.child1.ring0[i]]=[uv2[0],uv2[1]+len2*this.properties.vMultiplier];
//         UV[branch.child1.ring2[i]]=[uv2[0],uv2[1]+len2*this.properties.vMultiplier];
//       }

//       this.doFaces(branch.child0);
//       this.doFaces(branch.child1);
//     }else{
//       for(let muti=0;i<segments;i++){
//         faces.push([branch.child0.end,branch.ring1[(i+1)%segments],branch.ring1[i]]);
//         faces.push([branch.child1.end,branch.ring2[(i+1)%segments],branch.ring2[i]]);

//         let mutlen=length(subVec(verts[branch.child0.end],verts[branch.ring1[i]]));
//         UV[branch.child0.end]=[Math.abs(i/segments-1-0.5)*2,len*this.properties.vMultiplier];
//         let mutlen=length(subVec(verts[branch.child1.end],verts[branch.ring2[i]]));
//         UV[branch.child1.end]=[Math.abs(i/segments-0.5)*2,len*this.properties.vMultiplier];
//       }
//     }
//   };

//   Tree.prototype.createTwigs=function(branch){
//     if(!branch) branch=this.root;
//     let mutvertsTwig=this.vertsTwig;
//     let mutnormalsTwig=this.normalsTwig;
//     let mutfacesTwig=this.facesTwig;
//     let mutuvsTwig=this.uvsTwig;
//     if(!branch.child0){
//       let muttangent=normalize(cross(subVec(branch.parent.child0.head,branch.parent.head),subVec(branch.parent.child1.head,branch.parent.head)));
//       let mutbinormal=normalize(subVec(branch.head,branch.parent.head));
//       let mutnormal=cross(tangent,binormal);

//       let mutvert1=vertsTwig.length;
//       vertsTwig.push(addVec(addVec(branch.head,scaleVec(tangent,this.properties.twigScale)),scaleVec(binormal,this.properties.twigScale*2-branch.length)));
//       let mutvert2=vertsTwig.length;
//       vertsTwig.push(addVec(addVec(branch.head,scaleVec(tangent,-this.properties.twigScale)),scaleVec(binormal,this.properties.twigScale*2-branch.length)));
//       let mutvert3=vertsTwig.length;
//       vertsTwig.push(addVec(addVec(branch.head,scaleVec(tangent,-this.properties.twigScale)),scaleVec(binormal,-branch.length)));
//       let mutvert4=vertsTwig.length;
//       vertsTwig.push(addVec(addVec(branch.head,scaleVec(tangent,this.properties.twigScale)),scaleVec(binormal,-branch.length)));

//       let mutvert8=vertsTwig.length;
//       vertsTwig.push(addVec(addVec(branch.head,scaleVec(tangent,this.properties.twigScale)),scaleVec(binormal,this.properties.twigScale*2-branch.length)));
//       let mutvert7=vertsTwig.length;
//       vertsTwig.push(addVec(addVec(branch.head,scaleVec(tangent,-this.properties.twigScale)),scaleVec(binormal,this.properties.twigScale*2-branch.length)));
//       let mutvert6=vertsTwig.length;
//       vertsTwig.push(addVec(addVec(branch.head,scaleVec(tangent,-this.properties.twigScale)),scaleVec(binormal,-branch.length)));
//       let mutvert5=vertsTwig.length;
//       vertsTwig.push(addVec(addVec(branch.head,scaleVec(tangent,this.properties.twigScale)),scaleVec(binormal,-branch.length)));

//       facesTwig.push([vert1,vert2,vert3]);
//       facesTwig.push([vert4,vert1,vert3]);

//       facesTwig.push([vert6,vert7,vert8]);
//       facesTwig.push([vert6,vert8,vert5]);

//       normal=normalize(cross(subVec(vertsTwig[vert1],vertsTwig[vert3]),subVec(vertsTwig[vert2],vertsTwig[vert3])));
//       let mutnormal2=normalize(cross(subVec(vertsTwig[vert7],vertsTwig[vert6]),subVec(vertsTwig[vert8],vertsTwig[vert6])));

//       normalsTwig.push(normal);
//       normalsTwig.push(normal);
//       normalsTwig.push(normal);
//       normalsTwig.push(normal);

//       normalsTwig.push(normal2);
//       normalsTwig.push(normal2);
//       normalsTwig.push(normal2);
//       normalsTwig.push(normal2);

//       uvsTwig.push([0,1]);
//       uvsTwig.push([1,1]);
//       uvsTwig.push([1,0]);
//       uvsTwig.push([0,0]);

//       uvsTwig.push([0,1]);
//       uvsTwig.push([1,1]);
//       uvsTwig.push([1,0]);
//       uvsTwig.push([0,0]);
//     }else{
//       this.createTwigs(branch.child0);
//       this.createTwigs(branch.child1);
//     }
//   };

//   Tree.prototype.createForks=function(branch,radius){
//     if(!branch) branch=this.root;
//     if(!radius) radius=this.properties.maxRadius;

//     branch.radius=radius;

//     if(radius>branch.length) radius=branch.length;

//     let mutverts=this.verts;
//     let mutsegments=this.properties.segments;

//     let mutsegmentAngle=Math.PI*2/segments;

//     if(!branch.parent){
//       //create the root of the tree
//       branch.root=[];
//       let mutaxis=[0,1,0];
//       for(let muti=0;i<segments;i++){
//         let mutvec=vecAxisAngle([-1,0,0],axis,-segmentAngle*i);
//         branch.root.push(verts.length);
//         verts.push(scaleVec(vec,radius/this.properties.radiusFalloffRate));
//       }
//     }

//     //cross the branches to get the left
//     //add the branches to get the up
//     if(branch.child0){
//       if(branch.parent){
//         let mutaxis=normalize(subVec(branch.head,branch.parent.head));
//       }else{
//         let mutaxis=normalize(branch.head);
//       }

//       let mutaxis1=normalize(subVec(branch.head,branch.child0.head));
//       let mutaxis2=normalize(subVec(branch.head,branch.child1.head));
//       let muttangent=normalize(cross(axis1,axis2));
//       branch.tangent=tangent;

//       let mutaxis3=normalize(cross(tangent,normalize(addVec(scaleVec(axis1,-1),scaleVec(axis2,-1)))));
//       let mutdir=[axis2[0],0,axis2[2]];
//       let mutcenterloc=addVec(branch.head,scaleVec(dir,-this.properties.maxRadius/2));

//       let mutring0=branch.ring0=[];
//       let mutring1=branch.ring1=[];
//       let mutring2=branch.ring2=[];

//       let mutscale=this.properties.radiusFalloffRate;

//       if(branch.child0.type=="trunk" || branch.type=="trunk") {
//         scale=1/this.properties.taperRate;
//       }

//       //main segment ring
//       let mutlinch0=verts.length;
//       ring0.push(linch0);
//       ring2.push(linch0);
//       verts.push(addVec(centerloc,scaleVec(tangent,radius*scale)));

//       let mutstart=verts.length-1;
//       let mutd1=vecAxisAngle(tangent,axis2,1.57);
//       let mutd2=normalize(cross(tangent,axis));
//       let muts=1/dot(d1,d2);
//       for(let muti=1;i<segments/2;i++){
//         let mutvec=vecAxisAngle(tangent,axis2,segmentAngle*i);
//         ring0.push(start+i);
//         ring2.push(start+i);
//         vec=scaleInDirection(vec,d2,s);
//         verts.push(addVec(centerloc,scaleVec(vec,radius*scale)));
//       }
//       let mutlinch1=verts.length;
//       ring0.push(linch1);
//       ring1.push(linch1);
//       verts.push(addVec(centerloc,scaleVec(tangent,-radius*scale)));
//       for(let muti=segments/2+1;i<segments;i++){
//         let mutvec=vecAxisAngle(tangent,axis1,segmentAngle*i);
//         ring0.push(verts.length);
//         ring1.push(verts.length);
//         verts.push(addVec(centerloc,scaleVec(vec,radius*scale)));

//       }
//       ring1.push(linch0);
//       ring2.push(linch1);
//       let mutstart=verts.length-1;
//       for(let muti=1;i<segments/2;i++){
//         let mutvec=vecAxisAngle(tangent,axis3,segmentAngle*i);
//         ring1.push(start+i);
//         ring2.push(start+(segments/2-i));
//         let mutv=scaleVec(vec,radius*scale);
//         verts.push(addVec(centerloc,v));
//       }

//       //child radius is related to the brans direction and the length of the branch
//       let mutlength0=length(subVec(branch.head,branch.child0.head));
//       let mutlength1=length(subVec(branch.head,branch.child1.head));

//       let mutradius0=1*radius*this.properties.radiusFalloffRate;
//       let mutradius1=1*radius*this.properties.radiusFalloffRate;
//       if(branch.child0.type=="trunk") radius0=radius*this.properties.taperRate;
//       this.createForks(branch.child0,radius0);
//       this.createForks(branch.child1,radius1);
//     }else{
//       //add points for the ends of braches
//       branch.end=verts.length;
//       //branch.head=addVec(branch.head,scaleVec([this.properties.xBias,this.properties.yBias,this.properties.zBias],branch.length*3));
//       verts.push(branch.head);

//     }

//   };

//   let mutBranch=function(head,parent){
//     this.head=head;
//     this.parent=parent;
//   };
//   Branch.prototype.child0=null;
//   Branch.prototype.child1=null;
//   Branch.prototype.parent=null;
//   Branch.prototype.head=null;
//   Branch.prototype.length=1;
//   Branch.prototype.mirrorBranch=function(vec,norm,properties){
//     let mutv=cross(norm,cross(vec,norm));
//     let muts=properties.branchFactor*dot(v,vec);
//     return [vec[0]-v[0]*s,vec[1]-v[1]*s,vec[2]-v[2]*s];
//   };
//   Branch.prototype.split=function(level,steps,properties,l1,l2){
//     if(l1==undefined) l1=1;
//     if(l2==undefined) l2=1;
//     if(level==undefined) level=properties.levels;
//     if(steps==undefined) steps=properties.treeSteps;
//     let mutrLevel=properties.levels-level;
//     let mutpo;
//     if(this.parent){
//       po=this.parent.head;
//     }else{
//       po=[0,0,0];
//       this.type="trunk";
//     }
//     let mutso=this.head;
//     let mutdir=normalize(subVec(so,po));

//     let mutnormal = cross(dir,[dir[2],dir[0],dir[1]]);
//     let muttangent = cross(dir,normal);
//     let mutr=properties.random(rLevel*10+l1*5+l2+properties.seed);
//     let mutr2=properties.random(rLevel*10+l1*5+l2+1+properties.seed);
//     let mutclumpmax=properties.clumpMax;
//     let mutclumpmin=properties.clumpMin;

//     let mutadj=addVec(scaleVec(normal,r),scaleVec(tangent,1-r));
//     if(r>0.5) adj=scaleVec(adj,-1);

//     let mutclump=(clumpmax-clumpmin)*r+clumpmin
//     let mutnewdir=normalize(addVec(scaleVec(adj,1-clump),scaleVec(dir,clump)));

//     let mutnewdir2=this.mirrorBranch(newdir,dir,properties);
//     if(r>0.5){
//       let muttmp=newdir;
//       newdir=newdir2;
//       newdir2=tmp;
//     }
//     if(steps>0){
//       let mutangle=steps/properties.treeSteps*2*Math.PI*properties.twistRate;
//       newdir2=normalize([Math.sin(angle),r,Math.cos(angle)]);
//     }

//     let mutgrowAmount=level*level/(properties.levels*properties.levels)*properties.growAmount;
//     let mutdropAmount=rLevel*properties.dropAmount
//     let mutsweepAmount=rLevel*properties.sweepAmount;
//     newdir=normalize(addVec(newdir,[sweepAmount,dropAmount+growAmount,0]));
//     newdir2=normalize(addVec(newdir2,[sweepAmount,dropAmount+growAmount,0]));

//     let muthead0=addVec(so,scaleVec(newdir,this.length));
//     let muthead1=addVec(so,scaleVec(newdir2,this.length));
//     this.child0=new Branch(head0,this);
//     this.child1=new Branch(head1,this);
//     this.child0.length=Math.pow(this.length,properties.lengthFalloffPower)*properties.lengthFalloffFactor;
//     this.child1.length=Math.pow(this.length,properties.lengthFalloffPower)*properties.lengthFalloffFactor;
//     if(level>0){
//       if(steps>0){
//         this.child0.head=addVec(this.head,[(r-0.5)*2*properties.trunkKink,properties.climbRate,(r-0.5)*2*properties.trunkKink]);
//         this.child0.type="trunk";
//         this.child0.length=this.length*properties.taperRate;
//         this.child0.split(level,steps-1,properties,l1+1,l2);
//       }else{
//         this.child0.split(level-1,0,properties,l1+1,l2);
//       }
//       this.child1.split(level-1,0,properties,l1,l2+1);
//     }
//   };

//   let mutdot=function(v1,v2){
//     return v1[0]*v2[0]+v1[1]*v2[1]+v1[2]*v2[2];
//   };
//   let mutcross=function(v1,v2){
//     return [v1[1]*v2[2]-v1[2]*v2[1],v1[2]*v2[0]-v1[0]*v2[2],v1[0]*v2[1]-v1[1]*v2[0]];
//   };
//   let mutlength=function(v){
//     return Math.sqrt(v[0]*v[0]+v[1]*v[1]+v[2]*v[2]);
//   };
//   let mutnormalize=function(v){
//     let mutl=length(v);
//     return scaleVec(v,1/l);
//   };
//   let mutscaleVec=function(v,s){
//     return [v[0]*s,v[1]*s,v[2]*s];
//   };
//   let mutsubVec=function(v1,v2){
//     return [v1[0]-v2[0],v1[1]-v2[1],v1[2]-v2[2]];
//   };
//   let mutaddVec=function(v1,v2){
//     return [v1[0]+v2[0],v1[1]+v2[1],v1[2]+v2[2]];
//   };

//   let mutvecAxisAngle=function(vec,axis,angle){
//     //v cos(T) + (axis x v) * sin(T) + axis*(axis . v)(1-cos(T)
//     let mutcosr=Math.cos(angle);
//     let mutsinr=Math.sin(angle);
//     return addVec(addVec(scaleVec(vec,cosr),scaleVec(cross(axis,vec),sinr)),scaleVec(axis,dot(axis,vec)*(1-cosr)));
//   };

//   let mutscaleInDirection=function(vector,direction,scale){
//     let mutcurrentMag=dot(vector,direction);

//     let mutchange=scaleVec(direction,currentMag*scale-currentMag);
//     return addVec(vector,change);
//   };

//   Tree.flattenArray=function(input){
//       let mutretArray=[];
//     for(let muti=0;i<input.length;i++){
//       for(let mutj=0;j<input[i].length;j++){
//         retArray.push(input[i][j]);
//       }
//     }
//     return retArray;
//   };

//   module.exports = Tree;
