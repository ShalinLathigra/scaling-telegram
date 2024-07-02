use gl;

pub struct Triangle {
    // gl: gl::Gl,
    vertex_array_object: gl::types::GLuint,
}

impl Triangle {
    pub fn from_array(
        gl: &gl::Gl,
        vertices: &Vec<f32>
    ) -> Result<Triangle, String> {
        let mut vertex_buffer_object: gl::types::GLuint = 0;
        unsafe {
            // create one vertex buffer starting from vbo
            // vbo could also be an array of GLuints representing a larger set of objects
            gl.GenBuffers(1, &mut vertex_buffer_object);
        }
        unsafe {
            gl.BindBuffer(gl::ARRAY_BUFFER, vertex_buffer_object);
            gl.BufferData(
                gl::ARRAY_BUFFER, // target
                (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, // size of the buffer. # verts & f32 size
                vertices.as_ptr() as *const gl::types::GLvoid, // void pointer to the buffer data
                gl::STATIC_DRAW
            );
            gl.BindBuffer(gl::ARRAY_BUFFER, 0);
        }
        let mut vertex_array_object: gl::types::GLuint = 0;
        unsafe {
            gl.GenVertexArrays(1, &mut vertex_array_object);
        }
        unsafe {
            // need both vao and vbo for this
            gl.BindVertexArray(vertex_array_object);
            gl.BindBuffer(gl::ARRAY_BUFFER, vertex_buffer_object);

            // actual data layout for this buffer
            gl.EnableVertexAttribArray(0); // layout (location = 0) in the vertex shader. Can have multiple vertex attribute arrays
            gl.VertexAttribPointer(
                0, // index of generic vertex attribute layout (location = 0)
                3, // # attributes for this. [1-4], default value is 4. Color?
                gl::FLOAT, // type of data.
                gl::FALSE, // is this normalized? (Is the data passed in fixed or floating point)
                (6 * std::mem::size_of::<f32>()) as gl::types::GLint, // size of steps through the data block
                std::ptr::null() // offset of the first component. How/why would this be used?
            );

            // actual data layout for this buffer
            gl.EnableVertexAttribArray(1); // layout (location = 0) in the vertex shader. Can have multiple vertex attribute arrays
            gl.VertexAttribPointer(
                1, // index of generic vertex attribute layout (location = 0)
                3, // # attributes for this. [1-4], default value is 4. Color?
                gl::FLOAT, // type of data.
                gl::FALSE, // is this normalized? (Is the data passed in fixed or floating point)
                (6 * std::mem::size_of::<f32>()) as gl::types::GLint, // size of steps through the data block
                (3 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid // offset of the first component. Used to index into the set for defining vertex attributes

                /*
                in this example, vbo is laid out like :
                x,y,x,r,g,b,x,y,x,r,g,b,x,y,x,r,g,b,x,y,x,r,g,b
                stride 6 means that it is broken down into:
                x,y,x,r,g,b | x,y,x,r,g,b | x,y,x,r,g,b | x,y,x,r,g,b
                defining each attribute as is done above each having size == 3
                (x,y,x),(r,g,b) | (x,y,x),(r,g,b) | (x,y,x),(r,g,b) | (x,y,x),(r,g,b)
                */
            );

            // unbind buffers
            gl.BindBuffer(gl::ARRAY_BUFFER, 0);
            gl.BindVertexArray(0);
        }
        Ok(Triangle{vertex_array_object})
    }

    pub fn vao(&self) -> gl::types::GLuint {
        return self.vertex_array_object;
    }
}
