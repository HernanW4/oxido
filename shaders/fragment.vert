        #version 140
        
        in vec3 v_tex_cords;
        in vec3 my_color;
        out vec4 color;

        uniform sampler2D tex;
        void main(){
            color = vec4(my_color, 1.0);
        }
 
