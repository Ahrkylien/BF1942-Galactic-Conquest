subshader "bwing_cockpit_int_Material0" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular true;
        materialSpecular 0.8 0.8 1.0;
	materialSpecularPower 4.0;
	envmap true;
        transparent true;
        depthWrite false;
	materialDiffuse 0.588235 0.588235 0.588235;
	texture "texture/vehicles/bwing/bwing_cockpit_int";
}

