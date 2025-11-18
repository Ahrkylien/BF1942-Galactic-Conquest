subshader "flagbase_m1_Material0" "StandardMesh/Default"
{
	lighting false;
	lightingSpecular false;
	materialDiffuse 0.588235 0.588235 0.588235;
	transparent true;
        depthWrite false;
	alphaTestRef 0.7;
	texture "texture/flagbeam";
}

subshader "flagbase_m1_Material1" "StandardMesh/Default"
{
	lighting true;
	materialDiffuse 0.588235 0.588235 0.588235;
	lightingSpecular false;
	materialSpecular 0.5 0.5 0.5;
	materialSpecularPower 4.0;
	envmap true;
	texture "texture/flagbase";
}

