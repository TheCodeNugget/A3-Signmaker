use crate::io::*;
use std::io::{Read, Seek, Write, Error, BufReader, BufWriter};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use linked_hash_map::LinkedHashMap;

#[derive(Debug)]
pub struct Point {
    pub coords: (f32, f32, f32),
    pub flags: u32
}

#[derive(Debug)]
pub struct Vertex {
    pub point_index: u32,
    pub normal_index: u32,
    pub uv: (f32, f32)
}

#[derive(Debug)]
pub struct Face {
    pub vertices: Vec<Vertex>,
    pub flags: u32,
    pub texture: String,
    pub material: String
}

#[derive(Debug)]
pub struct LOD {
    pub version_major: u32,
    pub version_minor: u32,
    pub resolution: f32,
    pub points: Vec<Point>,
    pub face_normals: Vec<(f32, f32, f32)>,
    pub faces: Vec<Face>,
    pub taggs: LinkedHashMap<String, Box<[u8]>>
}

#[derive(Debug)]
pub struct P3D {
    pub version: u32,
    pub lods: Vec<LOD>
}

impl Point {
    #[allow(dead_code)]
    pub fn new() -> Point {
        Point { coords: (0.0, 0.0, 0.0), flags: 0 }
    }

    fn read<I: Read>(input: &mut I) -> Result<Point, Error> {
        Ok(Point {
            coords: (input.read_f32::<LittleEndian>()?, input.read_f32::<LittleEndian>()?, input.read_f32::<LittleEndian>()?),
            flags: input.read_u32::<LittleEndian>()?
        })
    }

    fn write<O: Write>(&self, output: &mut O) -> Result<(), Error> {
        output.write_f32::<LittleEndian>(self.coords.0)?;
        output.write_f32::<LittleEndian>(self.coords.1)?;
        output.write_f32::<LittleEndian>(self.coords.2)?;
        output.write_u32::<LittleEndian>(self.flags)?;
        Ok(())
    }
}

impl Vertex {
    #[allow(dead_code)]
    pub fn new() -> Vertex {
        Vertex { point_index: 0, normal_index: 0, uv: (0.0,0.0) }
    }

    fn read<I: Read>(input: &mut I) -> Result<Vertex, Error> {
        Ok(Vertex {
            point_index: input.read_u32::<LittleEndian>()?,
            normal_index: input.read_u32::<LittleEndian>()?,
            uv: (input.read_f32::<LittleEndian>()?, input.read_f32::<LittleEndian>()?)
        })
    }

    fn write<O: Write>(&self, output: &mut O) -> Result<(), Error> {
        output.write_u32::<LittleEndian>(self.point_index)?;
        output.write_u32::<LittleEndian>(self.normal_index)?;
        output.write_f32::<LittleEndian>(self.uv.0)?;
        output.write_f32::<LittleEndian>(self.uv.1)?;
        Ok(())
    }
}

impl Face {
    #[allow(dead_code)]
    pub fn new() -> Face {
        Face {
            vertices: Vec::with_capacity(4),
            flags: 0,
            texture: String::new(),
            material: String::new()
        }
    }

    fn read<I: Read>(input: &mut I) -> Result<Face, Error> {
        let num_verts = input.read_u32::<LittleEndian>()?;
        assert!(num_verts == 3 || num_verts == 4);

        let mut verts: Vec<Vertex> = Vec::with_capacity(num_verts as usize);
        for _i in 0..num_verts {
            verts.push(Vertex::read(input)?);
        }

        if num_verts == 3 {
            Vertex::read(input)?;
        }

        let flags = input.read_u32::<LittleEndian>()?;
        let texture = input.read_cstring()?;
        let material = input.read_cstring()?;

        Ok(Face {
            vertices: verts,
            flags: flags,
            texture: texture,
            material: material
        })
    }

    fn write<O: Write>(&self, output: &mut O) -> Result<(), Error> {
        output.write_u32::<LittleEndian>(self.vertices.len() as u32)?;

        for vert in &self.vertices {
            vert.write(output)?;
        }
        if self.vertices.len() == 3 {
            let vert = Vertex::new();
            vert.write(output)?;
        }

        output.write_u32::<LittleEndian>(self.flags)?;
        output.write_cstring(&self.texture)?;
        output.write_cstring(&self.material)?;
        Ok(())
    }
}

impl LOD {
    fn read<I: Read + Seek>(input: &mut I) -> Result<LOD, Error> {
        let mut buffer = [0; 4];
        input.read_exact(&mut buffer)?;
        assert_eq!(&buffer, b"P3DM");

        let version_major = input.read_u32::<LittleEndian>()?;
        let version_minor = input.read_u32::<LittleEndian>()?;

        let num_points = input.read_u32::<LittleEndian>()?;
        let num_face_normals = input.read_u32::<LittleEndian>()?;
        let num_faces = input.read_u32::<LittleEndian>()?;

        input.bytes().nth(3);

        let mut points: Vec<Point> = Vec::with_capacity(num_points as usize);
        let mut face_normals: Vec<(f32, f32, f32)> = Vec::with_capacity(num_face_normals as usize);
        let mut faces: Vec<Face> = Vec::with_capacity(num_faces as usize);

        for _i in 0..num_points {
            points.push(Point::read(input)?);
        }

        for _i in 0..num_face_normals {
            face_normals.push((input.read_f32::<LittleEndian>()?, input.read_f32::<LittleEndian>()?, input.read_f32::<LittleEndian>()?));
        }

        for _i in 0..num_faces {
            faces.push(Face::read(input)?);
        }

        input.read_exact(&mut buffer)?;
        assert_eq!(&buffer, b"TAGG");

        let mut taggs: LinkedHashMap<String, Box<[u8]>> = LinkedHashMap::new();

        loop {
            input.bytes().next();

            let name = input.read_cstring()?;
            let size = input.read_u32::<LittleEndian>()?;
            let mut buffer = vec![0; size as usize].into_boxed_slice();
            input.read_exact(&mut buffer)?;

            if name == "#EndOfFile#" { break; }

            taggs.insert(name, buffer);
        }

        let resolution = input.read_f32::<LittleEndian>()?;

        Ok(LOD {
            version_major: version_major,
            version_minor: version_minor,
            resolution: resolution,
            points: points,
            face_normals: face_normals,
            faces: faces,
            taggs: taggs
        })
    }

    fn write<O: Write>(&self, output: &mut O) -> Result<(), Error> {
        output.write_all(b"P3DM")?;
        output.write_u32::<LittleEndian>(self.version_major)?;
        output.write_u32::<LittleEndian>(self.version_minor)?;
        output.write_u32::<LittleEndian>(self.points.len() as u32)?;
        output.write_u32::<LittleEndian>(self.face_normals.len() as u32)?;
        output.write_u32::<LittleEndian>(self.faces.len() as u32)?;
        output.write_all(b"\0\0\0\0")?;

        for point in &self.points {
            point.write(output)?;
        }

        for normal in &self.face_normals {
            output.write_f32::<LittleEndian>(normal.0)?;
            output.write_f32::<LittleEndian>(normal.1)?;
            output.write_f32::<LittleEndian>(normal.2)?;
        }

        for face in &self.faces {
            face.write(output)?;
        }

        output.write_all(b"TAGG")?;

        for (name, buffer) in &self.taggs {
            output.write_all(&[1])?;
            output.write_cstring(name)?;
            output.write_u32::<LittleEndian>(buffer.len() as u32)?;
            output.write_all(buffer)?;
        }

        output.write_cstring("\x01#EndOfFile#")?;
        output.write_u32::<LittleEndian>(0)?;

        output.write_f32::<LittleEndian>(self.resolution)?;

        Ok(())
    }
}

impl P3D {
    #[allow(dead_code)]
    pub fn read<I: Read + Seek>(input: &mut I) -> Result<P3D, Error> {
        let mut reader = BufReader::new(input);

        let mut buffer = [0; 4];
        reader.read_exact(&mut buffer)?;
        assert_eq!(&buffer, b"MLOD");

        let version = reader.read_u32::<LittleEndian>()?;
        let num_lods = reader.read_u32::<LittleEndian>()?;
        let mut lods: Vec<LOD> = Vec::with_capacity(num_lods as usize);

        for _i in 0..num_lods {
            lods.push(LOD::read(&mut reader)?);
        }

        Ok(P3D {
            version: version,
            lods: lods
        })
    }

    #[allow(dead_code)]
    pub fn write<O: Write>(&self, output: &mut O) -> Result<(), Error> {
        let mut writer = BufWriter::new(output);

        writer.write_all(b"MLOD")?;
        writer.write_u32::<LittleEndian>(self.version)?;
        writer.write_u32::<LittleEndian>(self.lods.len() as u32)?;

        for lod in &self.lods {
            lod.write(&mut writer)?;
        }

        Ok(())
    }
}